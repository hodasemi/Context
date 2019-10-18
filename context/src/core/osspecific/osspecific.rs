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
    gamemode: Option<linux::gamemode::GameMode>,
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

        OsSpecific { gamemode: gamemode }
    }
}

#[cfg(target_os = "linux")]
impl std::fmt::Debug for OsSpecific {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let some = match &self.gamemode {
            Some(_) => "Some",
            None => "None",
        };

        write!(f, "OsSpecific {{ gamemode: {} }}", some)
    }
}

// ============================================================
// =====================    Others    =========================
// ============================================================
#[cfg(not(target_os = "linux"))]
pub struct OsSpecific;

#[cfg(not(target_os = "linux"))]
impl OsSpecific {
    pub fn new(_config: &OsSpecificConfig) -> OsSpecific {
        OsSpecific
    }

    pub fn enable(&self) {}

    pub fn disable(&self) {}
}

#[cfg(not(target_os = "linux"))]
impl std::fmt::Debug for OsSpecific {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "OsSpecific {{ }}")
    }
}
