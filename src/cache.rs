use std::fs::File;

use anyhow::{Context, Result};

use crate::paths;

pub enum Entry {
    /// A file open for reading that represents a cache hit
    Cached(File),

    /// A file open for writing that represents a cache miss
    /// that should be filled in
    Missing(File),
}

pub fn fetch(year: u32, day: u32) -> Result<Entry> {
    let cache_dir = paths::cache_directory()?;
    std::fs::create_dir_all(&cache_dir).with_context(|| {
        format!(
            "Failed to create cache directory structure: {}",
            cache_dir.display()
        )
    })?;

    let filename = format!("{}_{:02}.txt", year, day);
    let target_cache = cache_dir.join(filename);

    let result = if target_cache.exists() {
        let f = File::options()
            .read(true)
            .open(&target_cache)
            .with_context(|| format!("Failed to open cache file: {}", target_cache.display()))?;
        Entry::Cached(f)
    } else {
        let f = File::options()
            .create_new(true)
            .write(true)
            .open(&target_cache)
            .with_context(|| format!("Failed to create cache file: {}", target_cache.display()))?;
        Entry::Missing(f)
    };

    Ok(result)
}

pub fn force_write(year: u32, day: u32, contents: &[u8]) -> Result<()> {
    let cache_dir = paths::cache_directory()?;
    std::fs::create_dir_all(&cache_dir).with_context(|| {
        format!(
            "Failed to create cache directory structure: {}",
            cache_dir.display()
        )
    })?;

    let filename = format!("{}_{:02}.txt", year, day);
    let target_cache = cache_dir.join(filename);

    std::fs::write(target_cache, contents).context("Failed to write input file to cache")?;

    Ok(())
}
