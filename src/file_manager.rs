use dirs::home_dir;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::ffi::OsStr;
use std::fs::{self};
use std::path::PathBuf;

// More potentially important settings
// Font-Size (for questions etc)
// Base Color
// installation dest

#[derive(Serialize, Deserialize)]
struct Settings {
    app_directory: String,
    text_size: f32, // this is the size for the questions/answers of the cards
    answer_button_height: f32,
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
