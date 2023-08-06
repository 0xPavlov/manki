mod installer;
use eframe::{
    NativeOptions,
    epi::App,
    run_native,
    egui::CentralPanel,
    egui::Context,
    epi::Frame,
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

    fn name(&self) -> &str {
        return "Test";
    }

    fn update(&mut self, ctx: &Context, _frame: &Frame<>) { 
        CentralPanel::default().show(ctx, |ui| {
            ui.label("placeholder");
        });
    }
}
fn main() {
    let mut logger = Logger::new();
    installer::setup(&mut logger);
    logger.print_log();
    let app = Test;
    let native_options = NativeOptions::default();
    run_native(Box::new(app), native_options);
}
