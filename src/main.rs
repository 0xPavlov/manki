mod file_manager;
mod deck;
use eframe::{
    egui::{
        CentralPanel,
        Context,
        TextEdit,
        Ui,
        Button,
    },
    epi::{
        App,
        Frame,
    },
    NativeOptions,
    run_native,
};

use crate::deck::Deck;

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
}

pub enum State {
    MAINSCREEN,
    DECKSCREEN,
}

struct Manki {
    state: State,
    _decks: Vec<Deck>,

}

impl Manki {
    fn default() -> Manki {
        return Manki {state: State::MAINSCREEN, _decks: Vec::new()};
    }
}

impl App for Manki {
    
    fn name(&self) -> &str {
        return "Manki";
    }

    fn update(&mut self, ctx: &Context, _frame: &Frame<>) {
        CentralPanel::default().show(ctx, |ui|{
            match &self.state {
                State::MAINSCREEN => {
                    ui.label("MAINSCREEN");
                    if ui.button("switch").clicked() {
                        self.state = State::DECKSCREEN;
                    }
                }
                State::DECKSCREEN => {
                    render_deckscreen(ui);
                }
            }
        });
    }
}

fn render_deckscreen(ui: &mut Ui) {
    ui.label("Deckscreen");
}


fn main() { 
    let mut d = deck::Deck::empty(&"Test");
    let path = dirs::home_dir().unwrap().join("Manki");
    d.save_to_json(path);

    //let app = Manki::default();
    //let options = NativeOptions::default();
    //run_native(Box::new(app), options);
}
