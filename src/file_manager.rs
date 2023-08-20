use dirs::home_dir;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::{self};
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
struct Settings {
    home_directory: String,
}

pub(crate) fn app_directory() -> PathBuf {
    home_dir().unwrap().join("Manki")
}

pub(crate) fn decks_directory() -> PathBuf {
    app_directory().join("decks")
}

pub(crate) fn log_path() -> PathBuf {
    app_directory().join("manki.log")
}

pub(crate) fn list_files(directory_path: PathBuf) -> Result<Vec<PathBuf>, Box<dyn Error>> {






    let entries = fs::read_dir(directory_path)?;
    let mut files = Vec::new();

    for entry in entries {
        let path = entry?.path();

        if path.is_file() {
            files.push(path);
        }
    }
    return Ok(files);
}
