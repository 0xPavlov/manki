pub(crate) struct Logger {
    log: Vec<String>,
}

impl Logger {
    pub(crate) fn new() -> Logger {
        Logger { log: Vec::new() }
    }
}
