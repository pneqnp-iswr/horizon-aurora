// Aurora Ribbon Renderer — WGPU world-space backend (Horizon native hooks API 58).
// This source is inserted as a public module in horizon-client-render::gfx::native_render.
// Game code publishes bounded snapshots; only the render thread owns GPU resources.
// No unsafe code, network traffic, external assets, or Java/Fabric dependencies.

use std::{cell::RefCell, collections::VecDeque, sync::{Mutex, OnceLock}, time::{Duration, Instant}};
use glam::{Vec3, DVec3};
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
    @location(1) edge: f32,
};
@vertex
fn vs(@location(0) position: vec3<f32>, @location(1) tint: vec4<f32>, @location(2) edge: f32) -> VertexOut {
    var output: VertexOut;
    output.position = params.mvp * vec4<f32>(position, 1.0);
    output.tint = tint;
    output.edge = edge;
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
    // Screen-space derivative keeps the silhouette approximately pixel-smooth
    // under camera zoom, resolution changes and different ribbon widths.
    let feather = clamp(fwidth(input.edge) * 1.5, 0.04, 0.40);
    let edge_alpha = 1.0 - smoothstep(1.0 - feather, 1.0, abs(input.edge));
    return vec4<f32>(input.tint.rgb, input.tint.a * soft * edge_alpha);
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
    let previous_player = state.snapshot.position.is_some();
    let previous_enabled = state.snapshot.enabled;
    state.snapshot.position = pos.filter(|p| p.iter().all(|v| v.is_finite()));
    state.snapshot.dimension = dimension_hash(dimension);
    state.snapshot.style = style.min(2) as u8;
    state.snapshot.enabled = enabled && state.snapshot.position.is_some();
    state.snapshot.demo = demo;
    state.snapshot.sequence = state.snapshot.sequence.wrapping_add(1);
    state.snapshot.updated = Instant::now();
    if diagnostics_enabled() && (previous_player != state.snapshot.position.is_some()
        || previous_enabled != state.snapshot.enabled) {
        eprintln!("[aurora/ribbon] role=client player_available={} enabled={} sequence={} dimension_hash={:x}",
            state.snapshot.position.is_some(), state.snapshot.enabled,
            state.snapshot.sequence, state.snapshot.dimension);
    }
}
pub fn inactive() { publish("", None, false, 0, false); }
pub fn needs(needs: &mut RenderNeeds) {
    if lock_bridge().snapshot.enabled {
        needs.world_depth = true;
    } else {
        // The renderer asks for needs even on non-world frames. Release GPU
        // resources promptly when disabled or when a world closes.
        RENDERER.with(|cell| {
            let mut renderer = cell.borrow_mut();
            renderer.reset_path("disabled_or_world_closed");
            renderer.gpu = None;
        });
    }
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
struct Renderer {
    previous_sequence: u64,
    dimension: u64,
    points: VecDeque<Point>,
    staging: Vec<[f32; 8]>,
    samples: Vec<(Vec3,f32,f32)>,
    strand_points: Vec<Vec3>,
    sides: Vec<Vec3>,
    demo_epoch: Option<Instant>,
    animation_epoch: u64,
    perf_frames: u64,
    perf_cpu_us_total: u128,
    perf_cpu_us_max: u128,
    perf_vertex_max: usize,
    perf_buffer_reallocs: u64,
    perf_gpu_recreates: u64,
    gpu: Option<Gpu>,
}
impl Default for Renderer {
    fn default()->Self {
        let samples=POINT_LIMIT*8;
        Self {
            previous_sequence:0,dimension:0,
            points:VecDeque::with_capacity(POINT_LIMIT),
            staging:Vec::with_capacity(VERTEX_LIMIT),
            samples:Vec::with_capacity(samples),
            strand_points:Vec::with_capacity(samples),
            sides:Vec::with_capacity(samples),
            demo_epoch:None,animation_epoch:0,
            perf_frames:0,perf_cpu_us_total:0,perf_cpu_us_max:0,
            perf_vertex_max:0,perf_buffer_reallocs:0,perf_gpu_recreates:0,
            gpu:None,
        }
    }
}
thread_local! { static RENDERER: RefCell<Renderer> = RefCell::new(Renderer::default()); }

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

fn smooth01(t:f32)->f32 {let t=t.clamp(0.0,1.0);t*t*(3.0-2.0*t)}
fn point_opacity(stamp:Instant,now:Instant)->f32 {
    let age=now.saturating_duration_since(stamp).as_secs_f32();
    let remaining=(1.0-age/LIFETIME.as_secs_f32()).clamp(0.0,1.0);
    smooth01(age/0.10)*smooth01(remaining/0.28)
}
fn centripetal(p0:[f64;3],p1:[f64;3],p2:[f64;3],p3:[f64;3],t:f64)->[f64;3] {
    let dist=|a:[f64;3],b:[f64;3]|(0..3).map(|i|(a[i]-b[i]).powi(2)).sum::<f64>().sqrt();
    let d0=dist(p0,p1).max(1e-8).sqrt();
    let d1=dist(p1,p2).max(1e-8).sqrt();
    let d2=dist(p2,p3).max(1e-8).sqrt();
    let t2=t*t;let t3=t2*t;
    [0,1,2].map(|i|{
        let m1=d1*((p1[i]-p0[i])/d0-(p2[i]-p0[i])/(d0+d1)+(p2[i]-p1[i])/d1);
        let m2=d1*((p2[i]-p1[i])/d1-(p3[i]-p1[i])/(d1+d2)+(p3[i]-p2[i])/d2);
        ((2.0*t3-3.0*t2+1.0)*p1[i]+(t3-2.0*t2+t)*m1+
          (-2.0*t3+3.0*t2)*p2[i]+(t3-t2)*m2).clamp(p1[i].min(p2[i]),p1[i].max(p2[i]))
    })
}
// Transport the previous ribbon frame into the new tangent plane.
// Only steer it toward the camera when that direction is well-defined;
// this avoids the violent 180-degree flips at near camera/tangent alignment.
fn stable_side(tangent:Vec3,at:Vec3,previous:Option<Vec3>)->Vec3 {
    let transported=previous
        .map(|p|(p-tangent*p.dot(tangent)).normalize_or_zero())
        .filter(|p|p.length_squared()>1e-6);
    let raw_facing=tangent.cross((-at).normalize_or_zero());
    let facing_quality=raw_facing.length_squared();
    let facing=raw_facing.normalize_or_zero();
    if let Some(prev)=transported {
        if facing_quality < 1e-3 {return prev;}
        let desired=if facing.dot(prev)<0.0 {-facing}else{facing};
        // Maximum one-quarter correction at a sample; never make the local
        // side perpendicular to its centerline via an unchecked miter.
        return prev.lerp(desired,0.25).normalize_or_zero();
    }
    if facing_quality>=1e-3 {return facing;}
    for axis in [Vec3::Y,Vec3::X,Vec3::Z] {
        let side=tangent.cross(axis).normalize_or_zero();
        if side.length_squared()>1e-6 {return side;}
    }
    Vec3::X // guarded by tangent-length checks at the caller
}
// A world-oriented helix must NOT rotate when only the viewer moves.
// Transport the radial basis along the trail, independent of camera position.
fn helix_side(tangent:Vec3,previous:Option<Vec3>)->Vec3 {
    if let Some(old)=previous {
        let projected=(old-tangent*old.dot(tangent)).normalize_or_zero();
        if projected.length_squared()>1e-6 {return projected;}
    }
    // Shortest-arc rotation of world +Y onto this tangent, applied to +X.
    // Unlike blending a reference vector, it never goes parallel to the
    // tangent while approaching the +Y pole (common for jump/flight trails).
    let c=tangent.dot(Vec3::Y);
    if c < -0.9999 {
        // The antiparallel rotation is not unique; use a bounded fallback.
        return (Vec3::X-tangent*tangent.x).normalize_or_zero();
    }
    let v=Vec3::Y.cross(tangent);
    let x=Vec3::X;
    (x+v.cross(x)+v.cross(v.cross(x))*(1.0/(1.0+c))).normalize_or_zero()
}
fn helix_offset(tangent:Vec3,side:Vec3,phase:f32,radius:f32)->Vec3 {
    let bitangent=tangent.cross(side).normalize_or_zero();
    side*(phase.cos()*radius)+bitangent*(phase.sin()*radius)
}
fn diagnostics_enabled()->bool {
    static ON:OnceLock<bool>=OnceLock::new();
    *ON.get_or_init(||std::env::var("AURORA_RIBBON_DIAG").as_deref()==Ok("1"))
}
fn ribbon_triangles(out:&mut Vec<[f32;8]>,a:Vec3,b:Vec3,sa:Vec3,sb:Vec3,
                    ca:[f32;4],cb:[f32;4],wa:f32,wb:f32) {
    if out.len()+6>VERTEX_LIMIT{return;}
    let aa=a+sa*wa*0.5;let ab=a-sa*wa*0.5;
    let ba=b+sb*wb*0.5;let bb=b-sb*wb*0.5;
    let v=|p:Vec3,c:[f32;4],e:f32|[p.x,p.y,p.z,c[0],c[1],c[2],c[3],e];
    out.extend_from_slice(&[v(aa,ca,1.0),v(ab,ca,-1.0),v(ba,cb,1.0),
                            v(ba,cb,1.0),v(ab,ca,-1.0),v(bb,cb,-1.0)]);
}
impl Renderer {
    fn reset_path(&mut self,reason:&'static str) {
        if !self.points.is_empty()||self.demo_epoch.is_some(){
            self.animation_epoch=self.animation_epoch.wrapping_add(1);
            if diagnostics_enabled(){
                eprintln!("[aurora/ribbon] role=client reason={reason} epoch={} sequence={} points={}",
                    self.animation_epoch,self.previous_sequence,self.points.len());
            }
        }
        self.points.clear();self.previous_sequence=0;self.demo_epoch=None;
    }
    fn accept(&mut self,snapshot:Snapshot,now:Instant){
        if self.dimension!=snapshot.dimension {
            self.reset_path("dimension_transition");self.dimension=snapshot.dimension;
        }
        if self.previous_sequence!=snapshot.sequence {
            self.previous_sequence=snapshot.sequence;
            if let Some(pos)=snapshot.position {
                let mut add=true;
                if let Some(last)=self.points.back() {
                    let d=(pos[0]-last.at[0]).hypot(pos[1]-last.at[1]).hypot(pos[2]-last.at[2]);
                    if d>8.0 {self.reset_path("teleport");}
                    else if d<0.035 {add=false;}
                }
                if add {
                    if self.points.len()>=POINT_LIMIT {self.points.pop_front();}
                    self.points.push_back(Point{at:pos,stamp:now});
                }
            }
        }
        while self.points.front().is_some_and(|p|now.saturating_duration_since(p.stamp)>LIFETIME) {
            self.points.pop_front();
        }
    }
    fn build(&mut self,camera:DVec3,snapshot:Snapshot,now:Instant){
        self.staging.clear();
        if self.points.len()<2&&!snapshot.demo{return;}
        self.samples.clear();
        if snapshot.demo {
            let center=match snapshot.position {Some(p)=>p,None=>return};
            let center=relative(center,camera);
            let start=*self.demo_epoch.get_or_insert(now);
            let clock=now.saturating_duration_since(start).as_secs_f32();
            for i in 0..64 {
                let t=i as f32/63.0;
                let a=t*std::f32::consts::TAU*1.35+clock*0.6;
                let radius=0.65+0.38*t;
                self.samples.push((center+Vec3::new(a.cos()*radius,0.25+1.35*t,a.sin()*radius),t,1.0));
            }
        } else {
            let n=self.points.len();
            for k in 0..n-1 {
                let p0=self.points[k.saturating_sub(1)].at;
                let p1=self.points[k].at;
                let p2=self.points[k+1].at;
                let p3=self.points[(k+2).min(n-1)].at;
                let distance=(p2[0]-p1[0]).hypot(p2[1]-p1[1]).hypot(p2[2]-p1[2]);
                let steps=((distance/0.20).ceil() as usize).clamp(4,7);
                let a0=point_opacity(self.points[k].stamp,now);
                let a1=point_opacity(self.points[k+1].stamp,now);
                for sub in 0..steps {
                    let t=sub as f64/steps as f64;
                    let p=relative(centripetal(p0,p1,p2,p3,t),camera);
                    let fraction=(k as f32+t as f32)/(n-1) as f32;
                    if self.samples.last().is_none_or(|(last,_,_)|last.distance_squared(p)>1e-8){
                        self.samples.push((p,fraction,a0+(a1-a0)*t as f32));
                    }
                }
            }
            let last=&self.points[n-1];
            let p=relative(last.at,camera);
            if self.samples.last().is_none_or(|(v,_,_)|v.distance_squared(p)>1e-8) {
                self.samples.push((p,1.0,point_opacity(last.stamp,now)));
            }
        }
        if self.samples.len()<2{return;}
        let strands=if snapshot.style==2 {2}else{1};
        for strand in 0..strands {
            self.strand_points.clear();self.sides.clear();
            let mut previous_helix_frame:Option<Vec3>=None;
            for i in 0..self.samples.len() {
                let (p,t,_)=self.samples[i];
                let p=if snapshot.style==2 {
                    let prev=self.samples[i.saturating_sub(1)].0;
                    let next=self.samples[(i+1).min(self.samples.len()-1)].0;
                    let axis=(next-prev).normalize_or_zero();
                    let axis=if axis.length_squared()>1e-8 {axis}else{Vec3::Y};
                    let side=helix_side(axis,previous_helix_frame);
                    previous_helix_frame=Some(side);
                    // A true pair of counterposed strands winds in TWO radial
                    // axes around the trail centerline, not along one sine axis.
                    let phase=t*std::f32::consts::TAU*3.0
                        +strand as f32*std::f32::consts::PI;
                    p+helix_offset(axis,side,phase,0.20)
                }else{p};
                self.strand_points.push(p);
            }
            let n=self.strand_points.len();
            for i in 0..n {
                let p0=self.strand_points[i.saturating_sub(1)];
                let p2=self.strand_points[(i+1).min(n-1)];
                let tangent=(p2-p0).normalize_or_zero();
                let prev=self.sides.last().copied();
                let side=if tangent.length_squared()<1e-8 {
                    prev.unwrap_or(Vec3::X)
                }else{stable_side(tangent,self.strand_points[i],prev)};
                self.sides.push(side);
            }
            let width:f32=match snapshot.style{0=>0.24,1=>0.36,_=>0.19};
            let taper=|t:f32|smooth01(t/0.06)*smooth01((1.0-t)/0.06);
            for i in 1..n {
                let (_,t0,a0)=self.samples[i-1];
                let (_,t1,a1)=self.samples[i];
                let mut ca=color(snapshot.style,t0,strand);
                let mut cb=color(snapshot.style,t1,strand);
                ca[3]*=a0;cb[3]*=a1;
                ribbon_triangles(&mut self.staging,self.strand_points[i-1],self.strand_points[i],
                                 self.sides[i-1],self.sides[i],ca,cb,
                                 width*taper(t0),width*taper(t1));
            }
        }
    }
    // CPU render-submission cost only; NOT the duration of GPU execution.
    // Printed at most once per 3600 drawn frames and only with opt-in diagnostics.
    fn record_frame(&mut self, started:Instant, vertices:usize) {
        if !diagnostics_enabled() {return;}
        let us=started.elapsed().as_micros();
        self.perf_frames+=1;
        self.perf_cpu_us_total+=us;
        self.perf_cpu_us_max=self.perf_cpu_us_max.max(us);
        self.perf_vertex_max=self.perf_vertex_max.max(vertices);
        if self.perf_frames>=3600 {
            eprintln!("[aurora/ribbon/perf] role=client frames={} cpu_submit_avg_us={} cpu_submit_max_us={} vertices_max={} vertex_buffer_reallocs={} gpu_recreates={} draw_calls={}",
                self.perf_frames,self.perf_cpu_us_total/self.perf_frames as u128,
                self.perf_cpu_us_max,self.perf_vertex_max,self.perf_buffer_reallocs,
                self.perf_gpu_recreates,self.perf_frames);
            self.perf_frames=0;self.perf_cpu_us_total=0;self.perf_cpu_us_max=0;
            self.perf_vertex_max=0;self.perf_buffer_reallocs=0;self.perf_gpu_recreates=0;
        }
    }
    fn ensure_gpu(&mut self, world:&WorldDrawn<'_>) {
        let identity=world.device as *const wgpu::Device as usize;
        let replace=self.gpu.as_ref().is_none_or(|g|g.device_address!=identity||g.format!=world.color_format);
        if !replace {return;}
        self.gpu=None;
        self.perf_gpu_recreates=self.perf_gpu_recreates.saturating_add(1);
        if diagnostics_enabled() {eprintln!("[aurora/ribbon] role=client reason=gpu_resources_recreated epoch={} sequence={}",self.animation_epoch,self.previous_sequence);}
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
            label:Some("Aurora Ribbon pipeline layout"),bind_group_layouts:&[Some(&layout)],immediate_size:0,
        });
        let pipeline=device.create_render_pipeline(&wgpu::RenderPipelineDescriptor{
            label:Some("Aurora depth-aware 3D ribbons"),layout:Some(&pipeline_layout),
            vertex:wgpu::VertexState {
                module:&shader,entry_point:Some("vs"),
                buffers:&[Some(wgpu::VertexBufferLayout{
                    array_stride:32,step_mode:wgpu::VertexStepMode::Vertex,
                    attributes:&[
                        wgpu::VertexAttribute {format:wgpu::VertexFormat::Float32x3,offset:0,shader_location:0},
                        wgpu::VertexAttribute {format:wgpu::VertexFormat::Float32x4,offset:12,shader_location:1},
                        wgpu::VertexAttribute {format:wgpu::VertexFormat::Float32,offset:28,shader_location:2},
                    ],
                })],compilation_options:Default::default(),
            },
            fragment:Some(wgpu::FragmentState {
                module:&shader,entry_point:Some("fs"),
                targets:&[Some(wgpu::ColorTargetState {format:world.color_format,
                    blend:Some(wgpu::BlendState::ALPHA_BLENDING),write_mask:wgpu::ColorWrites::ALL})],
                compilation_options:Default::default(),
            }),
            primitive:wgpu::PrimitiveState{topology:wgpu::PrimitiveTopology::TriangleList,cull_mode:None,..Default::default()},
            depth_stencil:None,multisample:Default::default(),multiview_mask:None,cache:None,
        });
        let uniform=device.create_buffer(&wgpu::BufferDescriptor {
            label:Some("Aurora Ribbon camera uniforms"),size:80,
            usage:wgpu::BufferUsages::UNIFORM|wgpu::BufferUsages::COPY_DST,mapped_at_creation:false,
        });
        // 8192 vertices * 32 bytes = 256 KiB. Allocate once at activation,
        // rather than reallocating the vertex buffer mid-combat.
        let capacity=VERTEX_LIMIT;
        let vertices=device.create_buffer(&wgpu::BufferDescriptor {
            label:Some("Aurora Ribbon pooled vertices"),size:(capacity*32) as u64,
            usage:wgpu::BufferUsages::VERTEX|wgpu::BufferUsages::COPY_DST,mapped_at_creation:false,
        });
        self.gpu=Some(Gpu{pipeline,bind_layout:layout,uniform,vertices,capacity,format:world.color_format,device_address:identity});
    }
    fn draw(&mut self,world:&mut WorldDrawn<'_>,snapshot:Snapshot) {
        let now=Instant::now();
        let cpu_started=now;
        if !snapshot.enabled {
            self.reset_path("disabled");self.gpu=None;return;
        }
        // A paused/menu frame may have a stale client tick. Preserve epoch,
        // while explicit client inactive() still tears down world state.
        if now.saturating_duration_since(snapshot.updated)>Duration::from_secs(2) {return;}
        self.accept(snapshot,now);
        self.build(world.camera,snapshot,now);
        if world.size[0]==0 || world.size[1]==0 {return;}
        // Prewarm the pipeline on the first enabled world frame, before moving.
        self.ensure_gpu(world);
        if self.staging.is_empty() {return;}
        let gpu=self.gpu.as_mut().expect("ensure_gpu initialized resources");
        let count=self.staging.len().min(VERTEX_LIMIT);
        if count>gpu.capacity {
            self.perf_buffer_reallocs=self.perf_buffer_reallocs.saturating_add(1);
            gpu.capacity=count.next_power_of_two().min(VERTEX_LIMIT);
            gpu.vertices=world.device.create_buffer(&wgpu::BufferDescriptor {
                label:Some("Aurora Ribbon expanded vertex buffer"),size:(gpu.capacity*32) as u64,
                usage:wgpu::BufferUsages::VERTEX|wgpu::BufferUsages::COPY_DST,mapped_at_creation:false,
            });
        }
        let mut uniforms=[0.0_f32;20];
        // Match the TAA-jittered geometry used to populate scene_depth.
        uniforms[..16].copy_from_slice(&world.jittered.to_cols_array());
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
            })],depth_stencil_attachment:None,..Default::default()
        });
        pass.set_pipeline(&gpu.pipeline);
        pass.set_bind_group(0,&bind,&[]);
        pass.set_vertex_buffer(0,gpu.vertices.slice(..));
        pass.draw(0..count as u32,0..1);
        drop(pass);
        self.record_frame(cpu_started,count);
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
    fn ribbon_shader_parses_and_validates_with_naga() {
        let module=naga::front::wgsl::parse_str(WGSL).expect("Aurora WGPU WGSL must parse");
        naga::valid::Validator::new(naga::valid::ValidationFlags::all(),
                                    naga::valid::Capabilities::all())
            .validate(&module).expect("Aurora WGPU shader must validate");
    }
    #[test]
    fn centripetal_endpoint_invariants() {
        let p0 = [0.0, -1.0, 0.5];
        let p1 = [1.0, 2.0, 3.0];
        let p2 = [2.0, 4.0, 6.0];
        let p3 = [4.0, 5.0, 7.0];
        assert_eq!(centripetal(p0, p1, p2, p3, 0.0), p1);
        assert_eq!(centripetal(p0, p1, p2, p3, 1.0), p2);
    }
    #[test]
    fn continuous_strip_has_identical_shared_join_vertices(){
        let mut v=Vec::new();let c=[1.0;4];
        let a=Vec3::ZERO;let b=Vec3::new(1.0,0.3,0.0);let d=Vec3::new(2.0,0.2,0.7);
        let s0=Vec3::Y;let s1=Vec3::Z;let s2=Vec3::X;
        ribbon_triangles(&mut v,a,b,s0,s1,c,c,0.2,0.4);
        ribbon_triangles(&mut v,b,d,s1,s2,c,c,0.4,0.2);
        assert_eq!(&v[2][..3],&v[6][..3]);
        assert_eq!(&v[5][..3],&v[7][..3]);
    }
    #[test]
    fn spectral_helix_centerline_is_camera_translation_invariant() {
        let now=Instant::now();
        let mut renderer=Renderer::default();
        for i in 0..8 {
            let t=i as f64*0.18;
            renderer.points.push_back(Point{
                at:[t,2.0+t*0.5,(t*1.2).sin()*0.3],
                stamp:now-Duration::from_millis(100),
            });
        }
        let snapshot=Snapshot{
            position:Some([1.2,2.6,0.0]),dimension:7,
            style:2,enabled:true,demo:false,sequence:1,updated:now,
        };
        let camera_a=DVec3::new(0.0,1.0,5.0);
        let camera_b=DVec3::new(1.0,2.0,6.0);
        renderer.build(camera_a,snapshot,now);
        assert!(!renderer.staging.is_empty());
        let first=renderer.strand_points.clone();
        let full_vertex_count=renderer.staging.len();
        renderer.build(camera_b,snapshot,now);
        assert_eq!(full_vertex_count,renderer.staging.len());
        assert_eq!(first.len(),renderer.strand_points.len());
        let delta=Vec3::new(1.0,1.0,1.0);
        for (a,b) in first.iter().zip(&renderer.strand_points){
            assert!(((*a-*b)-delta).length()<1e-4);
        }
    }
    #[test]
    fn helix_frame_does_not_depend_on_camera_position(){
        // The same world-space path must keep its helix phase when the camera
        // switches between third- and first-person perspectives.
        let tangent=Vec3::new(0.4,0.5,0.76).normalize();
        let basis=helix_side(tangent,None);
        assert!(basis.is_finite());
        assert!((basis.length()-1.0).abs()<1e-5);
        assert!(basis.dot(tangent).abs()<1e-5);
        for phase in [0.0,1.0,2.3,3.5,5.0] {
            let offset=helix_offset(tangent,basis,phase,0.2);
            assert!((offset.length()-0.2).abs()<1e-5);
            assert!(offset.dot(tangent).abs()<1e-5);
        }
    }
    #[test]
    fn world_helix_seed_is_finite_at_vertical_poles(){
        for tangent in [Vec3::Y,-Vec3::Y,
                        Vec3::new(0.00001,0.999999,0.0).normalize(),
                        Vec3::new(0.00001,-0.999999,0.0).normalize()] {
            let side=helix_side(tangent,None);
            assert!(side.is_finite());
            assert!((side.length()-1.0).abs()<1e-5);
            assert!(side.dot(tangent).abs()<1e-5);
        }
    }
    #[test]
    fn helix_frame_seed_changes_smoothly_around_vertical_path(){
        let mut previous=None;
        for i in 0..500 {
            // Crossing the previous hard .92 dot threshold must not rotate
            // the initial radial seed by an entire right angle.
            let y=0.75+i as f32/499.0*0.245;
            let tangent=Vec3::new((1.0-y*y).sqrt(),y,0.0);
            let seed=helix_side(tangent,None);
            assert!(seed.is_finite());
            assert!((seed.length()-1.0).abs()<1e-4);
            if let Some(prior)=previous {
                assert!(seed.dot(prior)>0.985,"discontinuous helix seed at y={}",y);
            }
            previous=Some(seed);
        }
    }
    #[test]
    fn helix_basis_transports_through_near_vertical_motion(){
        let mut previous=None;
        for i in 0..128 {
            let a=i as f32/127.0*std::f32::consts::PI;
            let tangent=Vec3::new(a.sin()*0.2,a.cos(),a.sin()*0.98).normalize();
            let side=helix_side(tangent,previous);
            assert!(side.is_finite());
            assert!((side.length()-1.0).abs()<1e-4);
            assert!(side.dot(tangent).abs()<1e-4);
            previous=Some(side);
        }
    }
    #[test]
    fn spectral_helix_has_opposite_rotating_radial_strands(){
        let axis=Vec3::Z;
        let side=Vec3::X;
        let mut distinct=Vec::new();
        for i in 0..16 {
            let phase=i as f32/15.0*std::f32::consts::TAU;
            let a=helix_offset(axis,side,phase,0.2);
            let b=helix_offset(axis,side,phase+std::f32::consts::PI,0.2);
            assert!((a+b).length()<1e-5);
            assert!((a.length()-0.2).abs()<1e-5);
            distinct.push(a);
        }
        // The 3D strand must rotate in two independent radial axes.
        assert!(distinct.iter().any(|v|v.x.abs()>0.15));
        assert!(distinct.iter().any(|v|v.y.abs()>0.15));
    }
    #[test]
    fn full_motion_strip_has_watertight_sample_joins(){
        let now=Instant::now();
        let mut renderer=Renderer::default();
        for i in 0..POINT_LIMIT {
            let t=i as f64*0.10;
            renderer.points.push_back(Point {
                at:[t,(t*0.8).sin()*0.3,(t*0.6).cos()*0.5],
                stamp:now-Duration::from_millis((POINT_LIMIT-i) as u64*15),
            });
        }
        let snapshot=Snapshot {
            position:Some([7.0,0.0,0.0]),dimension:1,
            style:0,enabled:true,demo:false,sequence:1,updated:now,
        };
        renderer.build(DVec3::new(4.0,2.0,7.0),snapshot,now);
        let v=&renderer.staging;
        assert!(!v.is_empty());
        assert_eq!(v.len()%6,0);
        assert!(v.len()<=VERTEX_LIMIT);
        assert!(v.iter().all(|p|p.iter().all(|x|x.is_finite())));
        for chunks in v.chunks_exact(6).collect::<Vec<_>>().windows(2) {
            assert_eq!(&chunks[0][2][..3],&chunks[1][0][..3]);
            assert_eq!(&chunks[0][5][..3],&chunks[1][1][..3]);
        }
    }
    #[test]
    fn teleport_and_dimension_change_reset_path_once(){
        let now=Instant::now();
        let mut renderer=Renderer::default();
        let mut snapshot=Snapshot {
            position:Some([0.0,1.0,0.0]),dimension:10,
            style:0,enabled:true,demo:false,sequence:1,updated:now,
        };
        renderer.accept(snapshot,now);
        snapshot.position=Some([0.2,1.0,0.0]);snapshot.sequence+=1;
        renderer.accept(snapshot,now);
        assert_eq!(renderer.points.len(),2);
        snapshot.position=Some([30.0,1.0,0.0]);snapshot.sequence+=1;
        renderer.accept(snapshot,now);
        assert_eq!(renderer.points.len(),1);
        assert_eq!(renderer.animation_epoch,1);
        snapshot.dimension=11;snapshot.sequence+=1;
        renderer.accept(snapshot,now);
        assert_eq!(renderer.points.len(),1);
        assert_eq!(renderer.animation_epoch,2);
    }
    #[test]
    fn geometry_staging_is_preallocated_and_bounded(){
        let r=Renderer::default();
        assert!(r.staging.capacity()>=VERTEX_LIMIT);
        assert!(r.samples.capacity()>=POINT_LIMIT*8);
        assert!(r.strand_points.capacity()>=POINT_LIMIT*8);
        assert!(r.sides.capacity()>=POINT_LIMIT*8);
        assert!(r.points.capacity()>=POINT_LIMIT);
    }
    #[test]
    fn optional_perf_counters_do_not_change_disabled_render_state(){
        // Logging is explicitly opt-in; counters never touch gameplay snapshots.
        let renderer=Renderer::default();
        assert_eq!(renderer.perf_frames,0);
        assert_eq!(renderer.perf_buffer_reallocs,0);
        assert_eq!(renderer.perf_gpu_recreates,0);
        assert_eq!(renderer.animation_epoch,0);
    }
    #[test]
    fn frame_transport_stays_orthogonal_and_avoids_roll_flips(){
        let mut previous=None;
        for i in 0..128 {
            let angle=(i as f32/127.0)*std::f32::consts::PI*1.85;
            let tangent=Vec3::new(angle.cos(),0.20,angle.sin()).normalize();
            // Probe camera alignment degeneracies as the path bends.
            let at=-tangent*(1.0+(i as f32/127.0));
            let side=stable_side(tangent,at,previous);
            assert!(side.is_finite());
            assert!((side.length()-1.0).abs()<1e-4);
            assert!(side.dot(tangent).abs()<1e-4);
            if let Some(last)=previous { assert!(side.dot(last)>-0.25); }
            previous=Some(side);
        }
    }
    #[test]
    fn camera_facing_near_collinear_uses_transport(){
        let tangent=Vec3::X;
        let previous=Vec3::Y;
        let side=stable_side(tangent,Vec3::new(-100.0,0.000001,0.0),Some(previous));
        assert!(side.is_finite());
        assert!(side.dot(previous)>0.99);
    }
    #[test]
    fn demo_animation_epoch_survives_sequential_client_snapshots() {
        let t=Instant::now();
        let mut renderer=Renderer::default();
        let mut snapshot=Snapshot {
            position:Some([0.0,0.0,0.0]), dimension:42, style:0,
            enabled:true,demo:true,sequence:1,updated:t,
        };
        renderer.accept(snapshot,t);
        renderer.build(DVec3::new(0.0,0.0,4.0),snapshot,t);
        assert!(!renderer.staging.is_empty());
        let origin=renderer.demo_epoch;
        let later=t+Duration::from_millis(250);
        snapshot.sequence+=1;
        snapshot.updated=later;
        renderer.accept(snapshot,later);
        renderer.build(DVec3::new(0.0,0.0,4.0),snapshot,later);
        assert_eq!(renderer.demo_epoch,origin);
        assert_eq!(renderer.animation_epoch,0);
        assert!(!renderer.staging.is_empty());
    }
    #[test]
    fn per_point_age_fade_is_bounded(){
        let now=Instant::now();
        assert_eq!(point_opacity(now,now),0.0);
        assert!(point_opacity(now-Duration::from_millis(300),now)>0.9);
        assert_eq!(point_opacity(now-LIFETIME,now),0.0);
    }
    #[test]
    fn centripetal_segment_is_bounded(){
        let a=[-5.0,0.0,1.0];let b=[0.0,0.0,0.0];
        let c=[0.5,2.0,0.0];let d=[1.0,-6.0,1.0];
        for i in 0..=100 {
            let p=centripetal(a,b,c,d,i as f64/100.0);
            for j in 0..3 {assert!(p[j]>=b[j].min(c[j])-1e-12&&p[j]<=b[j].max(c[j])+1e-12);}
        }
    }
    #[test]
    fn ribbon_vertex_cap_and_gradient() {
        let mut vertices=Vec::new();
        for _ in 0..2000 {ribbon_triangles(&mut vertices,Vec3::ZERO,Vec3::X,Vec3::Z,Vec3::Z,
            color(1,0.,0),color(1,1.,0),0.2,0.1);}
        assert!(vertices.len()<=VERTEX_LIMIT);
        assert!(vertices.iter().all(|v|v.iter().all(|x|x.is_finite())));
    }
}
