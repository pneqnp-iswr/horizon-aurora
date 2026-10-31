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
