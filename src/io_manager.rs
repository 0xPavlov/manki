use crate::Settings;
use plist::Value;
use std::error::Error;
use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;
// More potentially important settings
// Font-Size (for questions etc)
// Base Color
// installation dest
//
//
pub struct PrefEntry {
    pub(crate) app_path: String,
}

impl PrefEntry {
    pub(crate) fn new(path: &str) -> PrefEntry {
        PrefEntry {
            app_path: path.to_string(),
        }
    }
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

pub(crate) fn check_for_preferences() -> Result<PrefEntry, Box<dyn Error>> {
    let entry: Value = Value::from_file(Settings::preferences_directory().join("manki.plist"))?;

    Ok(PrefEntry::new(
        entry
            .as_dictionary()
            .and_then(|dict| dict.get("app_path"))
            .and_then(|path| path.as_string())
            .unwrap(),
    ))
}

pub(crate) fn write_string_to_file(path: PathBuf, content: String) -> Result<(), Box<dyn Error>> {
    let mut file = File::create(path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}
