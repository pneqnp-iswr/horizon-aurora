// Aurora Ribbon Renderer — WGPU world-space backend (Horizon native hooks API 58).
// This source is inserted as a public module in horizon-client-render::gfx::native_render.
// Game code publishes bounded snapshots; only the render thread owns GPU resources.
// No unsafe code, network traffic, external assets, or Java/Fabric dependencies.

use std::{cell::RefCell, collections::VecDeque, sync::{Mutex, OnceLock}, time::{Duration, Instant}};
use glam::{Vec3, DVec3};
use wgpu::util::DeviceExt;
use super::{RenderNeeds, WorldDrawn};

const POINT_LIMIT: usize = 72;
const VERTEX_LIMIT: usize = 8192;
const LIFETIME: Duration = Duration::from_millis(1700);
const WGSL: &str = r#"
struct Params {
    mvp: mat4x4<f32>,
    frame: vec4<f32>,
};
@group(0) @binding(0) var<uniform> params: Params;
@group(0) @binding(1) var scene_depth: texture_depth_2d;

struct VertexOut {
    @builtin(position) position: vec4<f32>,
    @location(0) tint: vec4<f32>,
};
@vertex
fn vs(@location(0) position: vec3<f32>, @location(1) tint: vec4<f32>) -> VertexOut {
    var output: VertexOut;
    output.position = params.mvp * vec4<f32>(position, 1.0);
    output.tint = tint;
    return output;
}
@fragment
fn fs(input: VertexOut) -> @location(0) vec4<f32> {
    let dims = textureDimensions(scene_depth);
    let uv = input.position.xy / max(params.frame.xy, vec2<f32>(1.0));
    let texel = clamp(vec2<i32>(floor(uv * vec2<f32>(dims))),
                      vec2<i32>(0), vec2<i32>(dims) - vec2<i32>(1));
    // Horizon uses reversed depth: 0 = far/sky. Foreground geometry has HIGHER depth.
    let world_z = textureLoad(scene_depth, texel, 0);
    let separation = input.position.z - world_z;
    let soft = clamp((separation + 0.0015) * 180.0, 0.0, 1.0);
    return vec4<f32>(input.tint.rgb, input.tint.a * soft);
}
"#;

#[derive(Clone, Copy)]
struct Snapshot {
    position: Option<[f64; 3]>,
    dimension: u64,
    style: u8,
    enabled: bool,
    demo: bool,
    sequence: u64,
    updated: Instant,
}
struct Bridge { snapshot: Snapshot }
static BRIDGE: OnceLock<Mutex<Bridge>> = OnceLock::new();
fn bridge() -> &'static Mutex<Bridge> {
    BRIDGE.get_or_init(|| Mutex::new(Bridge { snapshot: Snapshot {
        position: None, dimension: 0, style: 0, enabled: false,
        demo: false, sequence: 0, updated: Instant::now(),
    }}))
}
fn lock_bridge() -> std::sync::MutexGuard<'static, Bridge> {
    bridge().lock().unwrap_or_else(std::sync::PoisonError::into_inner)
}
fn dimension_hash(id: &str) -> u64 {
    // Stable across frames, allocated nowhere; not a server-provided entity identifier.
    id.bytes().fold(0xcbf29ce484222325, |h, x| (h ^ u64::from(x)).wrapping_mul(0x100000001b3))
}
pub fn publish(dimension: &str, pos: Option<[f64; 3]>, enabled: bool, style: usize, demo: bool) {
    let mut state = lock_bridge();
    state.snapshot.position = pos.filter(|p| p.iter().all(|v| v.is_finite()));
    state.snapshot.dimension = dimension_hash(dimension);
    state.snapshot.style = style.min(2) as u8;
    state.snapshot.enabled = enabled && state.snapshot.position.is_some();
    state.snapshot.demo = demo;
    state.snapshot.sequence = state.snapshot.sequence.wrapping_add(1);
    state.snapshot.updated = Instant::now();
}
pub fn inactive() { publish("", None, false, 0, false); }
pub fn needs(needs: &mut RenderNeeds) {
    if lock_bridge().snapshot.enabled { needs.world_depth = true; }
}

#[derive(Clone, Copy)]
struct Point { at: [f64; 3], stamp: Instant }
struct Gpu {
    pipeline: wgpu::RenderPipeline,
    bind_layout: wgpu::BindGroupLayout,
    uniform: wgpu::Buffer,
    vertices: wgpu::Buffer,
    capacity: usize,
    format: wgpu::TextureFormat,
    device_address: usize, // identity comparison only; never dereferenced
}
#[derive(Default)]
struct Renderer {
    previous_sequence: u64,
    dimension: u64,
    points: VecDeque<Point>,
    staging: Vec<[f32; 8]>,
    samples: Vec<(Vec3,f32)>,
    gpu: Option<Gpu>,
}
thread_local! { static RENDERER: RefCell<Renderer> = RefCell::new(Renderer::default()); }

fn spline(a: f64, b: f64, c: f64, d: f64, t: f64) -> f64 {
    let t2 = t*t;
    0.5 * (2.0*b + (c-a)*t + (2.0*a-5.0*b+4.0*c-d)*t2
        + (-a+3.0*b-3.0*c+d)*t2*t)
}
fn hue_rgb(h: f32) -> [f32; 3] {
    let h = h.rem_euclid(1.0) * 6.0;
    let x = 1.0 - ((h % 2.0) - 1.0).abs();
    match h as u32 {
        0 => [1.0,x,0.0],1=>[x,1.0,0.0],2=>[0.0,1.0,x],
        3=>[0.0,x,1.0],4=>[x,0.0,1.0],_=>[1.0,0.0,x],
    }
}
fn color(style: u8, t: f32, strand: usize) -> [f32; 4] {
    match style {
        0 => [0.50 + 0.25*t, 0.75 - 0.30*t, 1.0, 0.80],
        1 => {
            let c = hue_rgb(t*0.70 + 0.18);
            [c[0], c[1], c[2], 0.86]
        }
        _ if strand==0 => [0.23,0.92,0.97,0.76],
        _ => [0.85,0.35,1.0,0.76],
    }
}
fn relative(p: [f64;3], camera:DVec3) -> Vec3 {
    // Subtract in f64 before narrowing: avoids far-from-origin wobble.
    Vec3::new((p[0]-camera.x) as f32,(p[1]-camera.y) as f32,(p[2]-camera.z) as f32)
}
fn ribbon_triangles(out: &mut Vec<[f32;8]>, a:Vec3, b:Vec3, side:Vec3,
                    ca:[f32;4], cb:[f32;4], width_a:f32, width_b:f32) {
    if out.len()+6 > VERTEX_LIMIT {return;}
    let aa=a+side*width_a*0.5;
    let ab=a-side*width_a*0.5;
    let ba=b+side*width_b*0.5;
    let bb=b-side*width_b*0.5;
    let v=|p:Vec3,c:[f32;4]| [p.x,p.y,p.z,c[0],c[1],c[2],c[3],0.0];
    out.extend_from_slice(&[v(aa,ca),v(ab,ca),v(ba,cb),v(ba,cb),v(ab,ca),v(bb,cb)]);
}
impl Renderer {
    fn reset_path(&mut self) { self.points.clear(); self.previous_sequence=0; }

    fn accept(&mut self, snapshot: Snapshot, now: Instant) {
        if self.dimension!=snapshot.dimension {self.reset_path();self.dimension=snapshot.dimension;}
        if self.previous_sequence!=snapshot.sequence {
            self.previous_sequence=snapshot.sequence;
            if let Some(pos)=snapshot.position {
                let mut add=true;
                if let Some(last)=self.points.back() {
                    let d = (pos[0]-last.at[0]).hypot(pos[1]-last.at[1]).hypot(pos[2]-last.at[2]);
                    if d > 8.0 {self.reset_path();}
                    else if d < 0.035 {add=false;}
                }
                if add {
                    if self.points.len()>=POINT_LIMIT {self.points.pop_front();}
                    self.points.push_back(Point {at:pos,stamp:now});
                }
            }
        }
        while self.points.front().is_some_and(|p| now.duration_since(p.stamp)>LIFETIME) {
            self.points.pop_front();
        }
    }

    fn build(&mut self, camera:DVec3, snapshot:Snapshot, now:Instant) {
        self.staging.clear();
        if self.points.len()<2 && !snapshot.demo {return;}
        self.samples.clear();
        if snapshot.demo {
            let center=match snapshot.position {Some(p)=>p,None=>return};
            let center=relative(center,camera);
            let clock=now.duration_since(snapshot.updated).as_secs_f32();
            for i in 0..32 {
                let t=i as f32/31.0;
                let a=t*std::f32::consts::TAU*1.35+clock*0.6;
                let radius=0.65+0.38*t;
                let p=center+Vec3::new(a.cos()*radius,0.25+1.35*t,a.sin()*radius);
                self.samples.push((p,t));
            }
        } else {
            let length=self.points.len();
            for k in 0..length-1 {
                let p0=self.points[k.saturating_sub(1)].at;
                let p1=self.points[k].at;
                let p2=self.points[k+1].at;
                let p3=self.points[(k+2).min(length-1)].at;
                for sub in 0..4 {
                    let t=sub as f64/4.0;
                    let xyz=[0,1,2].map(|n| spline(p0[n],p1[n],p2[n],p3[n],t));
                    self.samples.push((relative(xyz,camera), (k as f32+sub as f32/4.0)/(length-1) as f32));
                }
            }
            self.samples.push((relative(self.points[length-1].at,camera),1.0));
        }
        let count=if snapshot.style==2 {2}else{1};
        let origin=Vec3::ZERO;
        for strand in 0..count {
            let mut previous_side:Option<Vec3>=None;
            for i in 1..self.samples.len(){
                let (mut a,t0)=self.samples[i-1];let (mut b,t1)=self.samples[i];
                if snapshot.style==2 {
                    let wave0=(t0*std::f32::consts::TAU*3.0+strand as f32*std::f32::consts::PI).sin();
                    let wave1=(t1*std::f32::consts::TAU*3.0+strand as f32*std::f32::consts::PI).sin();
                    a += Vec3::new(wave0*0.18,wave0*0.24,-wave0*0.18);
                    b += Vec3::new(wave1*0.18,wave1*0.24,-wave1*0.18);
                }
                let tangent=(b-a).normalize_or_zero();
                if tangent.length_squared()<1e-8 {continue;}
                let toward=(origin-(a+b)*0.5).normalize_or_zero();
                let mut side=tangent.cross(toward).normalize_or_zero();
                if side.length_squared()<1e-6 {side=tangent.cross(Vec3::Y).normalize_or_zero();}
                if side.length_squared()<1e-6 {continue;}
                if let Some(prev)=previous_side {if side.dot(prev)<0.0 {side=-side;}}
                previous_side=Some(side);
                let width=match snapshot.style {0=>0.24,1=>0.36,_=>0.19};
                let taper=|t:f32| (t*5.0).min(1.0) * ((1.0-t)*4.0).min(1.0);
                let timefade=if snapshot.demo {1.0} else {
                    self.points.back().map(|p| (1.0-now.duration_since(p.stamp).as_secs_f32()/LIFETIME.as_secs_f32()).clamp(0.0,1.0)).unwrap_or(0.0)
                };
                let agefade=|t:f32| (0.18+0.82*t).min(1.0)*timefade;
                let mut ca=color(snapshot.style,t0,strand);
                let mut cb=color(snapshot.style,t1,strand);
                ca[3]*=agefade(t0);cb[3]*=agefade(t1);
                // World values are converted to camera-relative f32 for precision.
                ribbon_triangles(&mut self.staging,a,b,side,ca,cb,
                                width*taper(t0),width*taper(t1));
            }
        }
    }

    fn ensure_gpu(&mut self, world:&WorldDrawn<'_>) {
        let identity=world.device as *const wgpu::Device as usize;
        let replace=self.gpu.as_ref().is_none_or(|g|g.device_address!=identity||g.format!=world.color_format);
        if !replace {return;}
        self.gpu=None;
        let device=world.device;
        let shader=device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label:Some("Aurora Ribbon WGSL"),
            source:wgpu::ShaderSource::Wgsl(WGSL.into()),
        });
        let layout=device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label:Some("Aurora Ribbon world depth"),
            entries:&[
                wgpu::BindGroupLayoutEntry {binding:0,visibility:wgpu::ShaderStages::VERTEX_FRAGMENT,
                    ty:wgpu::BindingType::Buffer {ty:wgpu::BufferBindingType::Uniform,has_dynamic_offset:false,min_binding_size:None},count:None},
                wgpu::BindGroupLayoutEntry {binding:1,visibility:wgpu::ShaderStages::FRAGMENT,
                    ty:wgpu::BindingType::Texture {sample_type:wgpu::TextureSampleType::Depth,
                        view_dimension:wgpu::TextureViewDimension::D2,multisampled:false},count:None},
            ],
        });
        let pipeline_layout=device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label:Some("Aurora Ribbon pipeline layout"),bind_group_layouts:&[&layout],push_constant_ranges:&[],
        });
        let pipeline=device.create_render_pipeline(&wgpu::RenderPipelineDescriptor{
            label:Some("Aurora depth-aware 3D ribbons"),layout:Some(&pipeline_layout),
            vertex:wgpu::VertexState {
                module:&shader,entry_point:Some("vs"),
                buffers:&[wgpu::VertexBufferLayout{
                    array_stride:32,step_mode:wgpu::VertexStepMode::Vertex,
                    attributes:&[
                        wgpu::VertexAttribute {format:wgpu::VertexFormat::Float32x3,offset:0,shader_location:0},
                        wgpu::VertexAttribute {format:wgpu::VertexFormat::Float32x4,offset:12,shader_location:1},
                    ],
                }],compilation_options:Default::default(),
            },
            fragment:Some(wgpu::FragmentState {
                module:&shader,entry_point:Some("fs"),
                targets:&[Some(wgpu::ColorTargetState {format:world.color_format,
                    blend:Some(wgpu::BlendState::ALPHA_BLENDING),write_mask:wgpu::ColorWrites::ALL})],
                compilation_options:Default::default(),
            }),
            primitive:wgpu::PrimitiveState{topology:wgpu::PrimitiveTopology::TriangleList,cull_mode:None,..Default::default()},
            depth_stencil:None,multisample:Default::default(),multiview:None,cache:None,
        });
        let uniform=device.create_buffer(&wgpu::BufferDescriptor {
            label:Some("Aurora Ribbon camera uniforms"),size:80,
            usage:wgpu::BufferUsages::UNIFORM|wgpu::BufferUsages::COPY_DST,mapped_at_creation:false,
        });
        let capacity=1024;
        let vertices=device.create_buffer(&wgpu::BufferDescriptor {
            label:Some("Aurora Ribbon pooled vertices"),size:(capacity*32) as u64,
            usage:wgpu::BufferUsages::VERTEX|wgpu::BufferUsages::COPY_DST,mapped_at_creation:false,
        });
        self.gpu=Some(Gpu{pipeline,bind_layout:layout,uniform,vertices,capacity,format:world.color_format,device_address:identity});
    }
    fn draw(&mut self,world:&mut WorldDrawn<'_>,snapshot:Snapshot) {
        let now=Instant::now();
        if !snapshot.enabled || now.duration_since(snapshot.updated)>Duration::from_secs(2) {
            self.reset_path(); self.gpu=None;return;
        }
        self.accept(snapshot,now);
        self.build(world.camera,snapshot,now);
        if self.staging.is_empty() || world.size[0]==0 || world.size[1]==0 {return;}
        self.ensure_gpu(world);
        let gpu=self.gpu.as_mut().expect("ensure_gpu initialized resources");
        let count=self.staging.len().min(VERTEX_LIMIT);
        if count>gpu.capacity {
            gpu.capacity=count.next_power_of_two().min(VERTEX_LIMIT);
            gpu.vertices=world.device.create_buffer(&wgpu::BufferDescriptor {
                label:Some("Aurora Ribbon expanded vertex buffer"),size:(gpu.capacity*32) as u64,
                usage:wgpu::BufferUsages::VERTEX|wgpu::BufferUsages::COPY_DST,mapped_at_creation:false,
            });
        }
        let mut uniforms=[0.0_f32;20];
        uniforms[..16].copy_from_slice(&world.view_proj.to_cols_array());
        uniforms[16]=world.size[0] as f32;uniforms[17]=world.size[1] as f32;
        world.queue.write_buffer(&gpu.uniform,0,bytemuck::cast_slice(&uniforms));
        world.queue.write_buffer(&gpu.vertices,0,bytemuck::cast_slice(&self.staging[..count]));
        let bind=world.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label:Some("Aurora Ribbon frame depth"),layout:&gpu.bind_layout,
            entries:&[
                wgpu::BindGroupEntry{binding:0,resource:gpu.uniform.as_entire_binding()},
                wgpu::BindGroupEntry{binding:1,resource:wgpu::BindingResource::TextureView(world.depth)},
            ],
        });
        let mut pass=world.encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label:Some("Aurora Ribbon composite"),color_attachments:&[Some(wgpu::RenderPassColorAttachment {
                view:world.color_view,resolve_target:None,depth_slice:None,
                ops:wgpu::Operations {load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store},
            })],depth_stencil_attachment:None,..Default::default(),
        });
        pass.set_pipeline(&gpu.pipeline);
        pass.set_bind_group(0,&bind,&[]);
        pass.set_vertex_buffer(0,gpu.vertices.slice(..));
        pass.draw(0..count as u32,0..1);
    }
}
pub fn draw(world:&mut WorldDrawn<'_>) {
    let snapshot=lock_bridge().snapshot;
    RENDERER.with(|renderer|renderer.borrow_mut().draw(world,snapshot));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn cubic_endpoint_invariants() {
        assert_eq!(spline(0.,1.,2.,3.,0.),1.);
        assert_eq!(spline(0.,1.,2.,3.,1.),2.);
    }
    #[test]
    fn ribbon_vertex_cap_and_gradient() {
        let mut vertices=Vec::new();
        for _ in 0..2000 {ribbon_triangles(&mut vertices,Vec3::ZERO,Vec3::X,Vec3::Z,
            color(1,0.,0),color(1,1.,0),0.2,0.1);}
        assert!(vertices.len()<=VERTEX_LIMIT);
        assert!(vertices.iter().all(|v|v.iter().all(|x|x.is_finite())));
    }
}
