use std::fs::{
    metadata,
    create_dir,
};
use std::path::PathBuf;
use crate::Logger;
use serde::{Deserialize, Serialize};



#[derive(Serialize, Deserialize)]
struct Settings {
    home_directory: String,  
}




pub(crate) fn setup(logger: &mut Logger) {
}


pub(crate) fn is_directory(path: &PathBuf) -> bool {
    match metadata(path) {
        Ok(metadata) => {
            return metadata.is_dir();
        }
        Err(_) => {
            return false;
        }
    }
}


pub(crate) fn is_file(path: PathBuf) -> bool {
    match metadata(path) {
        Ok(data) => {
            return data.is_file();   
        }
        Err(_) => {
            return false;
        }
    }
}



pub(crate) fn create_directory(path: &PathBuf, logger: &mut Logger) {
    match create_dir(path) {
        Ok(()) => logger.log_success(&format!("Created the Application-Folder!")),
        Err(err) => logger.log_error(&format!("Could not create the Application-Folder at {} due to '{}'-error", path.to_str().unwrap(), err)),
    }
}
