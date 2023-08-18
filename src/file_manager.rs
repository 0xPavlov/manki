use dirs::home_dir;
use serde::{Deserialize, Serialize};
use std::fs::{self};
use std::io::{Error, ErrorKind};
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
struct Settings {
    home_directory: String,
}

pub(crate) fn app_directory() -> PathBuf {
    return home_dir().unwrap().join("Manki");
}

pub(crate) fn decks_directory() -> PathBuf {
    return app_directory().join("decks");
}

pub(crate) fn list_files(directory_path: PathBuf) -> Result<Vec<PathBuf>, Error> {
    if directory_path.is_file() {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            "Cannot retrieve files form a file!",
        ));
    }

    let entries = fs::read_dir(directory_path).unwrap();
    let mut files = Vec::new();

    for entry in entries {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => continue,
        };
        let path = entry.path();

        if path.is_file() {
            files.push(path);
        }
    }

    return Ok(files);
}
