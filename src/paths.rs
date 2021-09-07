use std::{fs, path::PathBuf};

use directories::ProjectDirs;
use once_cell::sync::Lazy;

pub(crate) static PROJECT_DIRS: Lazy<ProjectDirs> =
    Lazy::new(|| ProjectDirs::from("in", "kshlm", "cowin").unwrap_or_else(|| unreachable!()));
pub(crate) static CACHE: Lazy<PathBuf> = Lazy::new(|| {
    let cache = PROJECT_DIRS.cache_dir();
    fs::create_dir_all(cache).expect("Failed to create cache dir");
    cache.to_path_buf()
});
