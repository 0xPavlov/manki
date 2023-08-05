// Anything concering the installing is handeled here
use std::fs;
use std::path::PathBuf;
use crate::Logger;


pub(crate) fn setup(logger: &mut Logger) {
    let home_directory = dirs::home_dir().expect("failed");
    let directory_name = "Manki";
    let app_folder = home_directory.join(directory_name);

    if directory_exists(&app_folder) {
        logger.log_warning(&format!("Application-Folder already exists at {} (Creation skipped)", app_folder.to_str().unwrap()));
        return;
    }
    create_directory(&app_folder, logger);
}

pub(crate) fn directory_exists(path: &PathBuf) -> bool {
    return fs::metadata(path).is_ok();
}

pub(crate) fn create_directory(path: &PathBuf, logger: &mut Logger) {
    match fs::create_dir(path) {
        Ok(()) => logger.log_success(&format!("Succesfully created the Application-Folder!")),
        Err(err) => logger.log_error(&format!("Could not create the Application-Folder at {} due to '{}'-error", path.to_str().unwrap(), err)),
    }
}
