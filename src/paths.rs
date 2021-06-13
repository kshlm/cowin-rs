use std::{fs, path::PathBuf};

use directories::ProjectDirs;
use lazy_static::lazy_static;

lazy_static! {
    pub(crate) static ref PROJECT_DIRS: ProjectDirs =
        ProjectDirs::from("in", "kshlm", "cowin").unwrap_or_else(|| unreachable!());
    pub(crate) static ref CACHE: PathBuf = {
        let cache = PROJECT_DIRS.cache_dir();
        fs::create_dir_all(cache).expect("Failed to create cache dir");
        cache.to_path_buf()
    };
}
