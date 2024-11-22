use std::fs::File;
use std::path::PathBuf;

use eyre::{Result, WrapErr};

use crate::paths;

pub enum Entry {
    /// A file open for reading that represents a cache hit
    Cached(File),

    /// A filepath that represents a cache miss
    /// that should be filled in
    Missing(PathBuf),
}

pub fn fetch(year: u32, day: u32) -> Result<Entry> {
    let cache_dir = paths::cache_directory()?;
    std::fs::create_dir_all(&cache_dir).wrap_err_with(|| {
        format!(
            "Failed to create cache directory structure: {}",
            cache_dir.display()
        )
    })?;

    let filename = format!("{}_{:02}.txt", year, day);
    let target_cache = cache_dir.join(filename);

    if !target_cache.exists() {
        return Ok(Entry::Missing(target_cache));
    }

    let file_hit = File::options()
        .read(true)
        .open(&target_cache)
        .wrap_err_with(|| format!("Failed to open cache file: {}", target_cache.display()))?;

    Ok(Entry::Cached(file_hit))
}

pub fn force_write(year: u32, day: u32, contents: &[u8]) -> Result<()> {
    let cache_dir = paths::cache_directory()?;
    std::fs::create_dir_all(&cache_dir).wrap_err_with(|| {
        format!(
            "Failed to create cache directory structure: {}",
            cache_dir.display()
        )
    })?;

    let filename = format!("{}_{:02}.txt", year, day);
    let target_cache = cache_dir.join(filename);

    std::fs::write(target_cache, contents).wrap_err("Failed to write input file to cache")?;

    Ok(())
}
