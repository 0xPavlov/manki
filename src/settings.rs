use serde::{Deserialize, Serialize};
use std::{collections::HashMap, error::Error, fs, path::PathBuf};

use crate::io_manager::write_string_to_file;

#[derive(Serialize, Deserialize)]
pub(crate) struct Settings {
    pub(crate) app_directory: String,
}

impl Settings {
    pub(crate) fn new() -> Self {
        Self {
            app_directory: dirs::home_dir()
                .unwrap_or(PathBuf::from("Enter the Path here"))
                .to_string_lossy()
                .to_string(),
        }
    }

    pub(crate) fn from_file(directory: PathBuf) -> Result<Settings, Box<dyn Error>> {
        let file_contents = fs::read_to_string(directory.join("settings.json"))?;
        Ok(serde_json::from_str(&file_contents.as_str())?)
    }

    pub(crate) fn save_json_to_file(&mut self, directory: PathBuf) -> Result<(), Box<dyn Error>> {
        println!("{}", directory.to_string_lossy());
        write_string_to_file(
            directory.join("settings.json"),
            serde_json::to_string(self)?,
        )
    }

    pub(crate) fn register_preferences(app_path: &String) -> Result<(), Box<dyn Error>> {
        let mut map = HashMap::new();
        map.insert("app_path", app_path);
        let pval = plist::to_value(&map);
        let path = if cfg!(macos) {
            Self::preferences_directory().join("manki.plist")
        } else {
            Self::preferences_directory().join("manki.json")
        };
        pval.unwrap()
            .to_file_xml(Self::preferences_directory().join(path))?;
        Ok(())
    }

    pub(crate) fn preferences_directory() -> PathBuf {
        dirs::preference_dir().unwrap()
    }

    pub(crate) fn app_directory(&self) -> PathBuf {
        PathBuf::from(&self.app_directory)
    }

    pub(crate) fn decks_directory(&self) -> PathBuf {
        PathBuf::from(&self.app_directory).join("decks")
    }

    pub(crate) fn log_path(&self) -> PathBuf {
        PathBuf::from("manki.log")
    }
}
