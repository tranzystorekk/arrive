use std::path::PathBuf;

pub const BASE_DIR: &str = "arrive";
pub const STATE_FILE: &str = "arrive.toml";

pub fn cache_directory() -> Option<PathBuf> {
    dirs::cache_dir().map(|path| path.join(BASE_DIR))
}

pub fn state_directory() -> Option<PathBuf> {
    dirs::state_dir().map(|path| path.join(BASE_DIR))
}
