mod installer;
use eframe::{
    epi::App,
    run_native,
}; 

struct Logger {
    log: Vec<String>,
}

impl Logger {
    fn log_success(&mut self, to_add: &String) {
        self.log.push("Success: ".to_owned() + to_add);
    }

    fn log_warning(&mut self, to_add: &String) {
        self.log.push("Warning: ".to_owned() + to_add);
    }

    fn log_error(&mut self, to_add: &String) {
        self.log.push("Error: ".to_owned() + to_add);
    }

    fn new() -> Logger {
        Logger { log: Vec::new(), }
    }

    fn print_log(&mut self) {
        self.log.iter().for_each(|item| println!("{}", item));
    }
}

struct Test;

impl App for Test {

}

fn main() {
    let mut logger = Logger::new();
    installer::setup(&mut logger);
    logger.print_log();
    run_native(app, native_options);
}
