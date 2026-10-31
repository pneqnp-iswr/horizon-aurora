use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::{Mutex, OnceLock};

pub const MOD_ID: &str = "aurora";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ThemePreset {
    AuroraMidnight,
    NebulaViolet,
    CrimsonPulse,
    ArcticGlass,
    MinimalCarbon,
}

impl ThemePreset {
    pub const ORDER: [Self; 5] = [
        Self::AuroraMidnight,
        Self::NebulaViolet,
        Self::CrimsonPulse,
        Self::ArcticGlass,
        Self::MinimalCarbon,
    ];

    pub fn next(self) -> Self {
        let idx = Self::ORDER.iter().position(|it| *it == self).unwrap_or(0);
        Self::ORDER[(idx + 1) % Self::ORDER.len()]
    }

    pub fn name_en(self) -> &'static str {
        match self {
            Self::AuroraMidnight => "Aurora Midnight",
            Self::NebulaViolet => "Nebula Violet",
            Self::CrimsonPulse => "Crimson Pulse",
            Self::ArcticGlass => "Arctic Glass",
            Self::MinimalCarbon => "Minimal Carbon",
        }
    }

    pub fn name_ru(self) -> &'static str {
        match self {
            Self::AuroraMidnight => "Полярная полночь",
            Self::NebulaViolet => "Туманность фиолет",
            Self::CrimsonPulse => "Алый пульс",
            Self::ArcticGlass => "Арктическое стекло",
            Self::MinimalCarbon => "Минимальный карбон",
        }
    }

    pub fn accent_rgb(self) -> u32 {
        match self {
            Self::AuroraMidnight => 0x66CCFF,
            Self::NebulaViolet => 0xA97DFF,
            Self::CrimsonPulse => 0xFF5B87,
            Self::ArcticGlass => 0x7EDFE8,
            Self::MinimalCarbon => 0xA4B0B8,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HudPreset {
    Competitive,
    Cinematic,
    Minimal,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Locale {
    En,
    Ru,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuroraConfig {
    pub theme: ThemePreset,
    pub hud_preset: HudPreset,
    pub locale: Locale,
    pub reduced_motion: bool,
}

impl Default for AuroraConfig {
    fn default() -> Self {
        Self {
            theme: ThemePreset::AuroraMidnight,
            hud_preset: HudPreset::Competitive,
            locale: Locale::En,
            reduced_motion: false,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HudSnapshot {
    pub theme: ThemePreset,
    pub combo: u32,
    pub cps: u32,
    pub flash: f32,
    pub reduced_motion: bool,
    pub locale: Locale,
}

#[derive(Debug)]
pub struct AuroraCore {
    config: AuroraConfig,
    combo: u32,
    now: f32,
    last_click_at: Option<f32>,
    flash: f32,
    clicks: VecDeque<f32>,
}

impl AuroraCore {
    pub fn new(config: AuroraConfig) -> Self {
        Self {
            config,
            combo: 0,
            now: 0.0,
            last_click_at: None,
            flash: 0.0,
            clicks: VecDeque::with_capacity(16),
        }
    }

    pub fn config(&self) -> &AuroraConfig {
        &self.config
    }

    pub fn tick(&mut self, dt: f32) {
        let dt = dt.max(0.0).min(0.2);
        self.now += dt;
        while self
            .clicks
            .front()
            .is_some_and(|at| self.now - *at > 1.0)
        {
            self.clicks.pop_front();
        }
        if self
            .last_click_at
            .is_some_and(|last| self.now - last > 2.0)
        {
            self.combo = 0;
        }
        if self.config.reduced_motion {
            self.flash = if self.flash > 0.0 { 0.2 } else { 0.0 };
        } else {
            self.flash = (self.flash - dt * 2.4).max(0.0);
        }
    }

    pub fn register_left_click(&mut self) {
        self.combo = self.combo.saturating_add(1);
        self.last_click_at = Some(self.now);
        self.flash = 1.0;
        self.clicks.push_back(self.now);
    }

    pub fn cycle_theme(&mut self) {
        self.config.theme = self.config.theme.next();
    }

    pub fn toggle_reduced_motion(&mut self) {
        self.config.reduced_motion = !self.config.reduced_motion;
    }

    pub fn hud_snapshot(&self) -> HudSnapshot {
        HudSnapshot {
            theme: self.config.theme,
            combo: self.combo,
            cps: self.clicks.len() as u32,
            flash: self.flash,
            reduced_motion: self.config.reduced_motion,
            locale: self.config.locale,
        }
    }
}

fn state() -> &'static Mutex<AuroraCore> {
    static STATE: OnceLock<Mutex<AuroraCore>> = OnceLock::new();
    STATE.get_or_init(|| Mutex::new(AuroraCore::new(load_config().unwrap_or_default())))
}

fn with_state<T>(f: impl FnOnce(&mut AuroraCore) -> T) -> T {
    let mut guard = state().lock().unwrap_or_else(|poisoned| poisoned.into_inner());
    f(&mut guard)
}

pub fn hud_tick_snapshot(dt: f32) -> HudSnapshot {
    with_state(|core| {
        core.tick(dt);
        core.hud_snapshot()
    })
}

pub fn handle_key(key: &str, pressed: bool) {
    if !pressed {
        return;
    }
    with_state(|core| {
        let mut persist = false;
        match key {
            "key.mouse.left" => core.register_left_click(),
            "key.keyboard.f7" => {
                core.cycle_theme();
                persist = true;
            }
            "key.keyboard.f8" => {
                core.toggle_reduced_motion();
                persist = true;
            }
            _ => {}
        }
        if persist {
            let _ = save_config(core.config());
        }
    });
}

pub fn settings_buttons() -> Vec<String> {
    with_state(|core| {
        vec![
            format!("Theme: {}", core.config.theme.name_en()),
            format!(
                "Reduced motion: {}",
                if core.config.reduced_motion {
                    "On"
                } else {
                    "Off"
                }
            ),
        ]
    })
}

pub fn handle_settings_click(button: usize) {
    with_state(|core| {
        match button {
            0 => core.cycle_theme(),
            1 => core.toggle_reduced_motion(),
            _ => return,
        }
        let _ = save_config(core.config());
    });
}

pub fn save_config(config: &AuroraConfig) -> io::Result<()> {
    let Some(path) = config_path() else {
        return Ok(());
    };
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let json = serde_json::to_vec_pretty(config)
        .map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err))?;
    fs::write(path, json)
}

pub fn load_config() -> Option<AuroraConfig> {
    let path = config_path()?;
    load_config_path(&path).ok()
}

pub fn load_config_path(path: &Path) -> io::Result<AuroraConfig> {
    let data = fs::read(path)?;
    serde_json::from_slice(&data).map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err))
}

fn config_path() -> Option<PathBuf> {
    let base = env::var_os("HORIZON_NATIVE_DATA_DIR")?;
    Some(PathBuf::from(base).join(MOD_ID).join("config.json"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn theme_cycle_wraps() {
        let mut core = AuroraCore::new(AuroraConfig::default());
        for _ in 0..ThemePreset::ORDER.len() {
            core.cycle_theme();
        }
        assert_eq!(core.config.theme, ThemePreset::AuroraMidnight);
    }

    #[test]
    fn click_stream_produces_cps_and_combo_reset() {
        let mut core = AuroraCore::new(AuroraConfig::default());
        core.register_left_click();
        core.tick(0.2);
        core.register_left_click();
        core.tick(0.2);
        let snap = core.hud_snapshot();
        assert_eq!(snap.combo, 2);
        assert_eq!(snap.cps, 2);

        core.tick(2.1);
        assert_eq!(core.hud_snapshot().combo, 0);
    }

    #[test]
    fn reduced_motion_clamps_flash() {
        let mut config = AuroraConfig::default();
        config.reduced_motion = true;
        let mut core = AuroraCore::new(config);
        core.register_left_click();
        core.tick(0.05);
        assert_eq!(core.hud_snapshot().flash, 0.2);
    }

    #[test]
    fn json_roundtrip() {
        let config = AuroraConfig {
            theme: ThemePreset::CrimsonPulse,
            hud_preset: HudPreset::Minimal,
            locale: Locale::Ru,
            reduced_motion: true,
        };
        let encoded = serde_json::to_string(&config).expect("serialize config");
        let decoded: AuroraConfig = serde_json::from_str(&encoded).expect("deserialize config");
        assert_eq!(decoded, config);
    }

    #[test]
    fn theme_names_localized() {
        assert_eq!(ThemePreset::AuroraMidnight.name_en(), "Aurora Midnight");
        assert_eq!(ThemePreset::AuroraMidnight.name_ru(), "Полярная полночь");
    }
}
