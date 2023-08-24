use crate::file_manager;
use std::fs::File;
use std::io::Write;

pub(crate) struct Logger {
    pub _log: File,
}

impl Logger {
    pub(crate) fn new() -> Logger {
        Logger {
            _log: File::open(file_manager::log_path()).unwrap(),
        }
    }

    pub(crate) fn _log_error(&mut self, error: String) {
        let _ = self._log.write_all(format!("ERROR: {}", error).as_bytes());
    }

    pub(crate) fn _log_info(&mut self, info: String) {
        let _ = self._log.write_all(format!("INFO:  {}", info).as_bytes());
    }
}
