use std::fs::File;

use crate::paths;

pub enum Entry {
    /// A file open for reading that represents a cache hit
    Cached(File),

    /// A file open for writing that represents a cache miss
    /// that should be filled in
    Missing(File),
}

pub fn fetch(year: u32, day: u32) -> Entry {
    let cache_dir = paths::cache_directory().unwrap();
    std::fs::create_dir_all(&cache_dir).unwrap();

    let filename = format!("{}_{:02}.txt", year, day);
    let target_cache = cache_dir.join(filename);

    if target_cache.exists() {
        let f = File::options().read(true).open(target_cache).unwrap();
        Entry::Cached(f)
    } else {
        let f = File::options()
            .create_new(true)
            .write(true)
            .open(target_cache)
            .unwrap();
        Entry::Missing(f)
    }
}

pub fn force_write(year: u32, day: u32, contents: &[u8]) {
    let cache_dir = paths::cache_directory().unwrap();
    std::fs::create_dir_all(&cache_dir).unwrap();

    let filename = format!("{}_{:02}.txt", year, day);
    let target_cache = cache_dir.join(filename);

    std::fs::write(target_cache, contents).unwrap();
}
