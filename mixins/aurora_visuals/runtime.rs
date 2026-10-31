//! Aurora Visuals — dependency-free model, settings, animation and HUD composition.
//! This file is inserted as a module inside Horizon and independently tested by Cargo.
use std::cell::{Cell, RefCell};

pub const ID: &str = "aurora_visuals";
pub const NAME: &str = "Aurora Visuals";
pub const VERSION: &str = "0.3.0";
pub const DESCRIPTION: &str = "Native HUD, local aurora light trails, input flares, graphs and accessible visual settings. No combat automation.";
pub const CORNERS: [(&str, &str); 4] = [
    ("top_left", "Top left"), ("top_right", "Top right"),
    ("bottom_left", "Bottom left"), ("bottom_right", "Bottom right"),
];
pub const THEMES: [(&str, &str, u32); 7] = [
    ("violet", "Violet", 0xB78CFF),
    ("cyan", "Cyan", 0x55DCE9),
    ("amber", "Amber", 0xFFC66E),
    ("rose", "Rose", 0xFF7FAD),
    ("mint", "Mint", 0x67F0C2),
    ("electric", "Electric", 0xA4DBFF),
    ("scarlet", "Scarlet", 0xFF667D),
];
pub const PRESETS: [(&str, &str); 3] = [
    ("combat", "Combat"), ("minimal", "Minimal"), ("streamer", "Streamer"),
];
pub const PANEL_BG: u32 = 0xE010131E;
pub const ROW_BG: u32 = 0xA0283042;
pub const TEXT: u32 = 0xF3F3FF;
pub const MUTED: u32 = 0xA5AFC1;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Settings {
    pub shown: bool,
    pub corner: usize,
    pub theme: usize,
    pub preset: usize,
    pub fps: bool,
    pub cps: bool,
    pub keys: bool,
    pub speed: bool,
    pub coords: bool,
    pub graph: bool,
    pub feedback: bool,
    pub ambient: bool,
    pub trail: bool,
    pub click_flare: bool,
    pub adaptive_quality: bool,
    pub compact: bool,
    pub reduced_motion: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            shown: true, corner: 1, theme: 0, preset: 0,
            fps: true, cps: true, keys: true, speed: true, coords: false,
            graph: true, feedback: true, ambient: false, trail: false, click_flare: true, adaptive_quality: true, compact: false,
            reduced_motion: false,
        }
    }
}

impl Settings {
    pub fn for_preset(preset: usize) -> Self {
        let mut s = Self { preset: preset.min(2), ..Self::default() };
        match s.preset {
            1 => { // Focus on visibility, not decoration.
                s.cps = false; s.keys = false; s.speed = false;
                s.graph = false; s.feedback = false; s.compact = true;
            }
            2 => { // Never display location by default on a stream.
                s.graph = true; s.feedback = false;
            }
            _ => {}
        }
        s
    }
    pub fn apply_preset(&mut self, preset: usize) {
        let next = Self::for_preset(preset);
        self.preset = next.preset;
        self.fps = next.fps; self.cps = next.cps; self.keys = next.keys;
        self.speed = next.speed; self.coords = next.coords;
        self.graph = next.graph; self.feedback = next.feedback;
        self.compact = next.compact;
        // Personal visual/accessibility preferences survive preset changes.
    }
    pub fn read(get: impl Fn(&str) -> Option<String>) -> Self {
        let choose = |key: &str, names: &[&str], fallback: usize| {
            get(key).and_then(|v| names.iter().position(|name| *name == v.trim()))
                .unwrap_or(fallback)
        };
        let corner = choose("corner", &["top_left", "top_right", "bottom_left", "bottom_right"], 1);
        let theme = choose("theme", &["violet", "cyan", "amber", "rose", "mint", "electric", "scarlet"], 0);
        let preset = choose("preset", &["combat", "minimal", "streamer"], 0);
        let mut s = Self::for_preset(preset);
        s.corner = corner; s.theme = theme;
        let read_bool = |key: &str, fallback: bool| match get(key) {
            Some(v) if v.trim() == "true" => true,
            Some(v) if v.trim() == "false" => false,
            _ => fallback,
        };
        s.shown = read_bool("shown", s.shown);
        s.fps = read_bool("fps", s.fps);
        s.cps = read_bool("cps", s.cps);
        s.keys = read_bool("keys", s.keys);
        s.speed = read_bool("speed", s.speed);
        s.coords = read_bool("coords", s.coords);
        // v0.1's Streamer preset had coordinates ON. Never silently expose
        // that legacy location preference after upgrading to v0.2.
        if s.preset == 2 && get("schema").as_deref().map(str::trim) != Some("2") {
            s.coords = false;
        }
        s.graph = read_bool("graph", s.graph);
        s.feedback = read_bool("feedback", s.feedback);
        s.ambient = read_bool("ambient", s.ambient);
        s.trail = read_bool("trail", s.trail);
        s.click_flare = read_bool("click_flare", s.click_flare);
        s.adaptive_quality = read_bool("adaptive_quality", s.adaptive_quality);
        s.compact = read_bool("compact", s.compact);
        s.reduced_motion = read_bool("reduced_motion", s.reduced_motion);
        s
    }
    pub fn click(&mut self, index: usize) {
        match index {
            0 => self.shown = !self.shown,
            1 => self.corner = (self.corner + 1) % CORNERS.len(),
            2 => self.theme = (self.theme + 1) % THEMES.len(),
            3 => self.apply_preset((self.preset + 1) % PRESETS.len()),
            4 => self.fps = !self.fps,
            5 => self.cps = !self.cps,
            6 => self.keys = !self.keys,
            7 => self.speed = !self.speed,
            8 => self.coords = !self.coords,
            9 => self.graph = !self.graph,
            10 => self.feedback = !self.feedback,
            11 => self.ambient = !self.ambient,
            12 => self.compact = !self.compact,
            13 => self.reduced_motion = !self.reduced_motion,
            14 => self.trail = !self.trail,
            15 => self.click_flare = !self.click_flare,
            16 => self.adaptive_quality = !self.adaptive_quality,
            _ => {}
        }
    }
    pub fn labels(&self) -> Vec<String> {
        let on = |v| if v { "ON" } else { "OFF" };
        vec![
            format!("Aurora HUD: {}", on(self.shown)),
            format!("Corner: {}", CORNERS[self.corner].1),
            format!("Theme: {}", THEMES[self.theme].1),
            format!("Preset: {}", PRESETS[self.preset].1),
            format!("FPS: {}", on(self.fps)),
            format!("CPS: {}", on(self.cps)),
            format!("WASD: {}", on(self.keys)),
            format!("Speed: {}", on(self.speed)),
            format!("Coordinates (private): {}", on(self.coords)),
            format!("FPS graph: {}", on(self.graph)),
            format!("Input feedback (not hit detection): {}", on(self.feedback)),
            format!("Local ambient glow: {}", on(self.ambient)),
            format!("Compact: {}", on(self.compact)),
            format!("Reduced motion: {}", on(self.reduced_motion)),
            format!("Movement light trails: {}", on(self.trail)),
            format!("Click light flare (input only): {}", on(self.click_flare)),
            format!("Adaptive light quality: {}", on(self.adaptive_quality)),
        ]
    }
    pub fn stored(&self) -> Vec<(&'static str, String)> {
        let boolean = |v| (if v { "true" } else { "false" }).to_owned();
        vec![
            ("shown", boolean(self.shown)),
            ("corner", CORNERS[self.corner].0.into()),
            ("theme", THEMES[self.theme].0.into()),
            ("preset", PRESETS[self.preset].0.into()),
            ("fps", boolean(self.fps)),
            ("cps", boolean(self.cps)),
            ("keys", boolean(self.keys)),
            ("speed", boolean(self.speed)),
            ("coords", boolean(self.coords)),
            ("graph", boolean(self.graph)),
            ("feedback", boolean(self.feedback)),
            ("ambient", boolean(self.ambient)),
            ("compact", boolean(self.compact)),
            ("reduced_motion", boolean(self.reduced_motion)),
            ("trail", boolean(self.trail)),
            ("click_flare", boolean(self.click_flare)),
            ("adaptive_quality", boolean(self.adaptive_quality)),
            ("schema", "2".to_owned()),
        ]
    }
}

thread_local! {
    static SETTINGS: Cell<Option<Settings>> = const { Cell::new(None) };
    static MOTION: RefCell<Motion> = RefCell::new(Motion::default());
    static FRAMES: RefCell<FrameHistory> = RefCell::new(FrameHistory::default());
    static INPUTS: Cell<InputPulse> = const { Cell::new(InputPulse { left: 0, right: 0 }) };
    static WORLD_TICKS: Cell<u32> = const { Cell::new(0) };
    static TRAIL: RefCell<LightTrail> = RefCell::new(LightTrail::default());
    static QUALITY: RefCell<QualityGovernor> = RefCell::new(QualityGovernor::default());
}

pub fn settings(get: impl Fn(&str) -> Option<String>) -> Settings {
    SETTINGS.with(|cell| {
        let s = cell.get().unwrap_or_else(|| Settings::read(get));
        cell.set(Some(s));
        s
    })
}
pub fn set_settings(s: Settings) { SETTINGS.with(|cell| cell.set(Some(s))); }

/// 20 game ticks are approximately one second in a healthy world. This reports
/// *estimated* horizontal displacement per 20 ticks, not server-authoritative BPS.
#[derive(Clone, Debug, Default)]
pub struct Motion {
    previous: Option<(f64, f64)>,
    dimension: Option<String>,
    accumulation: f64,
    ticks: u32,
    pub bps: f64,
}
impl Motion {
    pub fn sample(&mut self, dimension: &str, pos: (f64, f64)) {
        if !pos.0.is_finite() || !pos.1.is_finite() { self.reset(); return; }
        if self.dimension.as_deref() != Some(dimension) {
            self.reset();
            self.dimension = Some(dimension.to_owned());
        }
        if let Some((x, z)) = self.previous {
            let step = (pos.0 - x).hypot(pos.1 - z);
            // Large single-tick jumps (teleports/respawns) do not make speed bars spike.
            if step <= 8.0 { self.accumulation += step; }
        }
        self.previous = Some(pos);
        self.ticks += 1;
        if self.ticks >= 20 {
            self.bps = self.accumulation;
            self.accumulation = 0.0;
            self.ticks = 0;
        }
    }
    pub fn reset(&mut self) { *self = Self::default(); }
}
pub fn sample_position(dimension: &str, x: f64, z: f64) {
    MOTION.with(|v| v.borrow_mut().sample(dimension, (x, z)));
}
pub fn reset_position() {
    MOTION.with(|v| v.borrow_mut().reset());
    TRAIL.with(|v| v.borrow_mut().clear());
}
pub fn movement_bps() -> f64 { MOTION.with(|v| v.borrow().bps) }

pub const HISTORY_CAP: usize = 32;
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct FrameHistory {
    samples: [u32; HISTORY_CAP],
    count: usize,
    head: usize,
    ticks: u8,
}
impl FrameHistory {
    pub fn tick(&mut self, fps: u32) {
        self.ticks = self.ticks.saturating_add(1);
        if self.ticks < 5 { return; }
        self.ticks = 0;
        self.samples[self.head] = fps.min(1000);
        self.head = (self.head + 1) % HISTORY_CAP;
        self.count = (self.count + 1).min(HISTORY_CAP);
    }
    pub fn values(&self) -> Vec<u32> {
        (0..self.count).map(|n| self.samples[(self.head + HISTORY_CAP - self.count + n) % HISTORY_CAP]).collect()
    }
}
pub fn frame_history() -> FrameHistory { FRAMES.with(|v| *v.borrow()) }

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct InputPulse { pub left: u8, pub right: u8 }
impl InputPulse {
    pub fn press(&mut self, left: bool) {
        if left { self.left = 8; } else { self.right = 8; }
    }
    pub fn tick(&mut self) {
        self.left = self.left.saturating_sub(1);
        self.right = self.right.saturating_sub(1);
    }
}
pub fn input_pulse() -> InputPulse { INPUTS.with(Cell::get) }
pub fn mouse_press(left: bool) { INPUTS.with(|v| { let mut pulse = v.get(); pulse.press(left); v.set(pulse); }); }
pub fn sample_frame(fps: u32) {
    let current = WORLD_TICKS.with(|v| { let n = v.get().wrapping_add(1); v.set(n); n });
    if current % 20 == 0 { QUALITY.with(|v| v.borrow_mut().observe(fps)); }
    FRAMES.with(|v| v.borrow_mut().tick(fps));
    INPUTS.with(|v| { let mut pulse = v.get(); pulse.tick(); v.set(pulse); });
}

/// Gentle ambient-light breathing; frozen when Reduced Motion is enabled.
/// Runs on client ticks, never on a wall clock or network data.
pub fn ambient_intensity(reduced_motion: bool) -> f32 {
    if reduced_motion { return 0.30; }
    let clock = WORLD_TICKS.with(Cell::get);
    let phase = (clock % 132) as f32 * (std::f32::consts::TAU / 132.0);
    0.31 + 0.09 * phase.sin()
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Reading {
    pub fps: u32,
    pub low_fps: u32,
    pub clicks: (u32, u32),
    pub bps: f64,
    pub position: Option<(f64, f64, f64)>,
    pub keys: [bool; 4],
}
#[derive(Clone, Debug, PartialEq)]
pub struct HudRow { pub label: &'static str, pub value: String }
pub fn rows(s: Settings, m: Reading) -> Vec<HudRow> {
    let mut out = Vec::with_capacity(7);
    if s.fps { out.push(HudRow { label: "FPS", value: format!("{}  /  1% {}", m.fps, m.low_fps) }); }
    if s.cps {
        out.push(HudRow { label: "LMB", value: format!("{} CPS", m.clicks.0) });
        out.push(HudRow { label: "RMB", value: format!("{} CPS", m.clicks.1) });
    }
    if s.speed { out.push(HudRow { label: "SPEED", value: format!("{:.1} BPS", m.bps.max(0.0)) }); }
    if s.keys {
        let keys = ["W", "A", "S", "D"].iter().zip(m.keys).map(|(name, pressed)| {
            if pressed { *name } else { "-" }
        }).collect::<Vec<_>>().join("  ");
        out.push(HudRow { label: "MOVE", value: keys });
    }
    if s.coords {
        if let Some((x, y, z)) = m.position {
            out.push(HudRow { label: "XYZ", value: format!("{x:.0}  {y:.0}  {z:.0}") });
        }
    }
    out
}

#[derive(Clone, Debug, PartialEq)]
pub enum Shape {
    Fill([f32; 4], u32),
    Text(String, [f32; 2], u32, bool),
}
#[derive(Clone, Debug, PartialEq)]
pub struct Panel { pub width: f32, pub height: f32, pub shapes: Vec<Shape> }

/// Compose HUD in local coordinates, using Horizon's actual font measurements.
/// All allocations are bounded by 32 history bars and seven stat rows.
pub fn panel(s: Settings, reading: Reading, history: FrameHistory, inputs: InputPulse,
             measure: &dyn Fn(&str) -> f32) -> Panel {
    let rows = rows(s, reading);
    let header = if s.compact { 25.0 } else { 31.0 };
    let line = if s.compact { 13.0 } else { 16.0 };
    let label_w = rows.iter().map(|r| measure(r.label)).fold(0.0_f32, f32::max);
    let value_w = rows.iter().map(|r| measure(&r.value)).fold(0.0_f32, f32::max);
    let title = "AURORA  /  HORIZON";
    let width = (label_w + value_w + 40.0).max(measure(title) + 24.0)
        .max(if s.compact { 140.0 } else { 175.0 });
    let graph_h = if s.graph { 42.0 } else { 0.0 };
    let feedback_h = if s.feedback { 21.0 } else { 0.0 };
    let height = header + line * rows.len() as f32 + graph_h + feedback_h + 5.0;
    let accent = THEMES[s.theme].2;
    let mut shapes = Vec::with_capacity(16 + rows.len() * 3 + HISTORY_CAP + 5);
    let mut fill = |rect: [f32; 4], color: u32| shapes.push(Shape::Fill(rect, color));
    fill([0.0, 0.0, width, height], PANEL_BG);
    fill([0.0, 0.0, width, 2.0], 0xFF000000 | accent);
    fill([0.0, 2.0, 2.0, height - 2.0], 0xAA000000 | accent);
    fill([width - 12.0, 9.0, 4.0, 4.0], 0xFF000000 | accent);
    drop(fill);
    shapes.push(Shape::Text(title.into(), [10.0, 7.0], accent, true));
    if !s.compact {
        shapes.push(Shape::Text(format!("{}  |  0.3", PRESETS[s.preset].1.to_uppercase()),
            [10.0, 20.0], MUTED, false));
    }
    for (i, row) in rows.iter().enumerate() {
        let y = header + i as f32 * line;
        if i % 2 == 0 {
            shapes.push(Shape::Fill([6.0, y, width - 12.0, line - 1.0], ROW_BG));
        }
        shapes.push(Shape::Text(row.label.into(), [10.0, y + 3.0], MUTED, false));
        let vx = (width - 10.0 - measure(&row.value)).max(14.0 + label_w);
        shapes.push(Shape::Text(row.value.clone(), [vx, y + 3.0], TEXT, true));
    }
    let mut y = header + rows.len() as f32 * line;
    if s.graph {
        shapes.push(Shape::Text("FPS HISTORY", [10.0, y + 4.0], MUTED, false));
        let gy = y + 16.0;
        let gx = 10.0;
        let gw = width - 20.0;
        let gh = 21.0;
        shapes.push(Shape::Fill([gx, gy, gw, gh], 0x701C2333));
        let values = history.values();
        let max = values.iter().copied().max().unwrap_or(60).max(30);
        // Show newest sample at the right edge, without reading future frames.
        let bw = (gw / HISTORY_CAP as f32).max(1.0);
        let offset = HISTORY_CAP - values.len();
        for (i, fps) in values.iter().enumerate() {
            let fraction = (*fps as f32 / max as f32).clamp(0.0, 1.0);
            let bar_h = (gh * fraction).max(1.0);
            shapes.push(Shape::Fill(
                [gx + (offset + i) as f32 * bw, gy + gh - bar_h, (bw - 1.0).max(1.0), bar_h],
                0xD0000000 | accent,
            ));
        }
        y += graph_h;
    }
    if s.feedback {
        shapes.push(Shape::Text("INPUT FX", [10.0, y + 5.0], MUTED, false));
        let right = width - 10.0;
        for (i, level) in [inputs.left, inputs.right].iter().enumerate() {
            let x = right - (2 - i) as f32 * 30.0;
            shapes.push(Shape::Fill([x, y + 7.0, 26.0, 6.0], 0x8030394A));
            let illuminated = if s.reduced_motion { if *level > 0 { 26.0 } else { 0.0 } }
                else { 26.0 * *level as f32 / 8.0 };
            if illuminated > 0.0 {
                shapes.push(Shape::Fill([x, y + 7.0, illuminated, 6.0], 0xFF000000 | accent));
            }
        }
    }
    if s.feedback && !s.reduced_motion {
        draw_pulse_ornament(&mut shapes, width - 27.0, height - 20.0, accent, inputs.left.max(inputs.right));
    }
    Panel { width, height, shapes }
}


/// Heuristic FPS proxy, not a GPU timing estimate. Reduces the number of
/// optional local lights only after sustained low FPS and restores them
/// slowly to avoid visible flicker. Never affects combat or game simulation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct QualityGovernor { pub level: u8, low: u8, high: u8 }
impl Default for QualityGovernor {
    fn default() -> Self { Self { level: 3, low: 0, high: 0 } }
}
impl QualityGovernor {
    pub fn observe(&mut self, fps: u32) {
        if fps < 55 {
            self.low = self.low.saturating_add(1);
            self.high = 0;
            if self.low >= 2 { self.level = self.level.saturating_sub(1); self.low = 0; }
        } else if fps > 110 {
            self.high = self.high.saturating_add(1);
            self.low = 0;
            if self.high >= 6 { self.level = (self.level + 1).min(3); self.high = 0; }
        } else { self.low = 0; self.high = 0; }
    }
}
pub fn trail_limit(adaptive: bool) -> usize {
    if adaptive { QUALITY.with(|v| usize::from(v.borrow().level)) }
    else { 3 }
}

/// Tiny fixed-size world-space trail. Only local player coordinates enter this
/// ring buffer. Teleports and dimension switches discard all old points.
/// Nothing is sent to servers or hidden through blocks: Horizon depth-tests
/// its own terrain normally and uses these as local lighting requests.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LightPoint { pub pos: [f64; 3], pub age: u8 }

#[derive(Clone, Debug, Default)]
pub struct LightTrail {
    dimension: Option<String>,
    last: Option<[f64; 3]>,
    points: Vec<LightPoint>,
    tick: u32,
}
impl LightTrail {
    const MAX_POINTS: usize = 12;
    pub fn clear(&mut self) { *self = Self::default(); }
    pub fn sample(&mut self, dimension: &str, pos: [f64; 3]) {
        if !pos.iter().all(|v| v.is_finite()) { self.clear(); return; }
        if self.dimension.as_deref() != Some(dimension) {
            self.clear(); self.dimension = Some(dimension.to_owned());
        }
        if let Some(last) = self.last {
            let dist = (pos[0]-last[0]).hypot(pos[1]-last[1]).hypot(pos[2]-last[2]);
            if dist > 8.0 { self.clear(); self.dimension = Some(dimension.to_owned()); }
        }
        self.last = Some(pos);
        self.tick = self.tick.wrapping_add(1);
        for p in &mut self.points { p.age = p.age.saturating_add(1); }
        self.points.retain(|p| p.age < 25);
        if self.tick % 3 != 0 { return; }
        // Limit emitters at a standstill; avoid a pile of lights in one place.
        let dist = self.points.last().map(|p| {
            (pos[0]-p.pos[0]).hypot(pos[1]-p.pos[1]).hypot(pos[2]-p.pos[2])
        }).unwrap_or(1.0);
        if dist < 0.28 { return; }
        if self.points.len() == Self::MAX_POINTS { self.points.remove(0); }
        self.points.push(LightPoint { pos, age: 0 });
    }
    pub fn lights(&self) -> Vec<LightPoint> {
        // Bounded most recent three, omitting any step older than 16 ticks.
        self.points.iter().rev().filter(|p| p.age <= 16).take(3).copied().collect()
    }
}
pub fn trail_sample(dimension: &str, xyz: [f64; 3]) {
    TRAIL.with(|v| v.borrow_mut().sample(dimension, xyz));
}
pub fn trail_lights() -> Vec<LightPoint> { TRAIL.with(|v| v.borrow().lights()) }

/// Centered screen-space preview ornament, drawn without any shader API.
/// This is local input animation and is deliberately not called a hit marker.
pub fn draw_pulse_ornament(out: &mut Vec<Shape>, x: f32, y: f32, accent: u32, strength: u8) {
    if strength == 0 { return; }
    let intensity = (strength as f32 / 8.0).clamp(0.0, 1.0);
    let alpha = ((intensity * 180.0) as u32) << 24;
    let color = alpha | accent;
    for n in 0..3 {
        let r = 4.0 + n as f32 * 4.0 + (8 - strength) as f32 * 0.7;
        let cx = x - r; let cy = y - r;
        out.push(Shape::Fill([cx, cy, r * 2.0, 1.0], color));
        out.push(Shape::Fill([cx, cy + r * 2.0, r * 2.0, 1.0], color));
        out.push(Shape::Fill([cx, cy, 1.0, r * 2.0], color));
        out.push(Shape::Fill([cx + r * 2.0, cy, 1.0, r * 2.0], color));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn measure(s: &str) -> f32 { s.len() as f32 * 6.0 }

    #[test]
    fn defaults_and_old_settings_remain_compatible() {
        let s = Settings::read(|_| None);
        assert_eq!(s, Settings::default());
        let legacy = Settings::read(|k| match k {
            "theme" => Some("cyan".into()), "preset" => Some("streamer".into()),
            "corner" => Some("bottom_right".into()),
            "coords" => Some("true".into()), // v0.1 previously wrote this.
            _ => None,
        });
        assert_eq!((legacy.theme, legacy.preset, legacy.corner), (1, 2, 3));
        assert!(!legacy.coords); // v0.1 Streamer location is auto-migrated to OFF.
    }
    #[test]
    fn settings_round_trip() {
        let mut s = Settings::default();
        for i in 0..s.labels().len() { s.click(i); }
        let stored = s.stored();
        let back = Settings::read(|k| stored.iter().find(|(key, _)| *key == k).map(|(_, v)| v.clone()));
        assert_eq!(s, back);
    }
    #[test]
    fn preset_preserves_visual_preferences() {
        let mut s = Settings { theme: 4, corner: 3, ambient: true, reduced_motion: true,
            ..Settings::default() };
        s.apply_preset(1);
        assert!(!s.graph && !s.feedback && s.compact);
        s.apply_preset(2);
        assert_eq!((s.theme, s.corner, s.ambient, s.reduced_motion), (4, 3, true, true));
        assert!(!s.coords);
    }
    #[test]
    fn world_transition_and_teleport_do_not_spike_speed() {
        let mut m = Motion::default();
        for i in 0..20 { m.sample("overworld", (i as f64 * 0.25, 0.0)); }
        assert!((m.bps - 4.75).abs() < 1e-7);
        m.sample("overworld", (2000.0, 0.0));
        for _ in 0..19 { m.sample("overworld", (2000.0, 0.0)); }
        assert_eq!(m.bps, 0.0);
        m.sample("nether", (0.0, 0.0));
        assert_eq!(m.bps, 0.0);
        for _ in 0..19 { m.sample("nether", (0.0, 0.0)); }
        assert_eq!(m.bps, 0.0);
        m.sample("nether", (f64::INFINITY, 0.0));
        assert_eq!(m.bps, 0.0);
    }
    #[test]
    fn fps_history_is_bounded_and_ordered_after_wrap() {
        let mut history = FrameHistory::default();
        for v in 1..=40 { for _ in 0..5 { history.tick(v); } }
        assert_eq!(history.values(), (9..=40).collect::<Vec<u32>>());
    }
    #[test]
    fn input_is_feedback_not_forged_hit_data() {
        let mut pulse = InputPulse::default();
        pulse.press(true);
        assert_eq!((pulse.left, pulse.right), (8, 0));
        for _ in 0..10 { pulse.tick(); }
        assert_eq!(pulse.left, 0);
        let result = rows(Settings::default(), Reading { fps: 200, low_fps: 100, ..Reading::default() });
        assert_eq!(result.len(), 5);
        assert!(result.iter().all(|r| !r.label.contains("TARGET") && !r.label.contains("HP")));
    }
    #[test]
    fn all_themes_presets_draw_finite_bounded_panels() {
        for preset in 0..PRESETS.len() {
            for theme in 0..THEMES.len() {
                let mut s = Settings { theme, ..Settings::for_preset(preset) };
                s.coords = true;
                let p = panel(s, Reading { fps: 280, low_fps: 190, bps: 6.4,
                    clicks: (12, 7), position: Some((-125000.0, 64.0, 9000.0)),
                    keys: [true, false, true, false] }, FrameHistory::default(),
                    InputPulse { left: 7, right: 5 }, &measure);
                assert!(p.width.is_finite() && p.height.is_finite());
                assert!(p.width > 130.0 && p.height > 15.0);
                for shape in &p.shapes {
                    match shape {
                        Shape::Fill([x, y, w, h], _) => {
                            assert!(*x >= 0.0 && *y >= 0.0 && *w >= 0.0 && *h >= 0.0);
                            assert!(*x + *w <= p.width + 0.01);
                            assert!(*y + *h <= p.height + 0.01);
                        }
                        Shape::Text(_, [x, y], _, _) => {
                            assert!(*x >= 0.0 && *y >= 0.0 && *y <= p.height);
                        }
                    }
                }
            }
        }
    }
    #[test]
    fn ambient_animation_has_known_bounds_and_reduced_motion_is_fixed() {
        assert_eq!(ambient_intensity(true), 0.30);
        for _ in 0..1000 {
            sample_frame(120);
            let i = ambient_intensity(false);
            assert!((0.21..=0.41).contains(&i), "intensity: {i}");
        }
    }

    #[test]
    fn trail_is_bounded_and_teleport_safe() {
        let mut t = LightTrail::default();
        for i in 0..120 { t.sample("overworld", [i as f64 * 0.12, 65.0, 0.0]); }
        assert!(t.points.len() <= LightTrail::MAX_POINTS);
        assert!(t.lights().len() <= 3);
        assert!(t.lights().iter().all(|p| p.age <= 16));
        t.sample("overworld", [9999.0, 65.0, 0.0]);
        assert!(t.points.len() <= 1);
        t.sample("nether", [0.0, 80.0, 0.0]);
        assert!(t.points.is_empty());
        t.sample("nether", [f64::NAN, 80.0, 0.0]);
        assert!(t.last.is_none());
    }
    #[test]
    fn quality_hysteresis_prevents_frame_to_frame_flicker() {
        let mut q = QualityGovernor::default();
        q.observe(54); assert_eq!(q.level, 3);
        q.observe(54); assert_eq!(q.level, 2);
        q.observe(70); q.observe(40); assert_eq!(q.level, 2);
        q.observe(40); assert_eq!(q.level, 1);
        for _ in 0..5 { q.observe(130); } assert_eq!(q.level, 1);
        q.observe(130); assert_eq!(q.level, 2);
        for _ in 0..60 { q.observe(20); } assert_eq!(q.level, 0);
    }
    #[test]
    fn pulse_ornament_is_bounded_and_off_when_idle() {
        let mut shapes = Vec::new();
        draw_pulse_ornament(&mut shapes, 30.0, 30.0, 0xA4DBFF, 0);
        assert!(shapes.is_empty());
        draw_pulse_ornament(&mut shapes, 30.0, 30.0, 0xA4DBFF, 8);
        assert_eq!(shapes.len(), 12);
        assert!(shapes.iter().all(|sh| match sh {
            Shape::Fill([x, y, w, h], _) => *x >= 0.0 && *y >= 0.0 && *x+*w <= 60.0 && *y+*h <= 60.0,
            _ => false,
        }));
    }

    #[test]
    fn malformed_settings_use_safe_defaults() {
        let s = Settings::read(|_| Some("invalid".into()));
        assert_eq!(s, Settings::default());
    }
}
