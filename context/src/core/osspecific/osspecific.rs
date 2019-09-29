#[derive(Default, Debug)]
pub struct OsSpecificConfig {
    pub enable_game_mode: bool,
}

// ============================================================
// ======================    Linux    =========================
// ============================================================
#[cfg(target_os = "linux")]
use crate::core::osspecific::linux;

#[cfg(target_os = "linux")]
pub struct OsSpecific {
    _gamemode: Option<linux::gamemode::GameMode>,
}

#[cfg(target_os = "linux")]
impl OsSpecific {
    pub fn new(config: &OsSpecificConfig) -> OsSpecific {
        let gamemode = if config.enable_game_mode {
            match linux::gamemode::GameMode::new() {
                Ok(gamemode) => Some(gamemode),
                Err(msg) => {
                    println!("{}", msg);
                    None
                }
            }
        } else {
            None
        };

        OsSpecific {
            _gamemode: gamemode,
        }
    }
}

// ============================================================
// =====================    Others    =========================
// ============================================================
#[cfg(not(target_os = "linux"))]
pub struct OsSpecific;

#[cfg(not(target_os = "linux"))]
impl OsSpecific {
    pub fn new(config: &OsSpecificConfig) -> OsSpecific {
        OsSpecific
    }

    pub fn enable(&self) {}

    pub fn disable(&self) {}
}
