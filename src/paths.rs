use std::path::PathBuf;

use anyhow::{anyhow, Result};

pub const BASE_DIR: &str = "arrive";
pub const STATE_FILE: &str = "arrive.toml";

pub fn cache_directory() -> Result<PathBuf> {
    dirs::cache_dir()
        .map(|path| path.join(BASE_DIR))
        .ok_or_else(|| anyhow!("Failed to locate cache directory for your platform"))
}

pub fn state_directory() -> Result<PathBuf> {
    dirs::state_dir()
        .or_else(|| dirs::home_dir().map(|p| p.join(".local/state")))
        .map(|path| path.join(BASE_DIR))
        .ok_or_else(|| anyhow!("Failed to locate state directory for your platform"))
}
