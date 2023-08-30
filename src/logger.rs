use crate::file_manager;
use std::fs::File;
use std::io::Write;

pub(crate) struct Logger {
    pub log: File,
}

impl Logger {
    pub(crate) fn new() -> Logger {
        Logger {
            log: File::create(file_manager::log_path()).unwrap(),
        }
    }

    pub(crate) fn log_error(&mut self, error: String) {
        self.log
            .write_all(format!("ERROR: {}\n", error).as_bytes())
            .unwrap();
    }

    pub(crate) fn _log_info(&mut self, info: String) {
        let _ = self.log.write_all(format!("INFO:  {}", info).as_bytes());
    }
}
