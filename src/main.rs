mod file_manager;
mod deck;
mod logger;
mod icons;

use eframe::{
    egui::{
        CentralPanel,
        Context,
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
use icons::trash_can;
use crate::deck::Deck;
use crate::logger::Logger;

pub enum State {
    HOMESCREEN,
    STUDYSCREEN,
}

struct Manki {
    state: State,
    decks: Vec<Deck>,
    logger: Logger,
}

impl Manki {
    fn default() -> Manki {
        return Manki {
            state: State::HOMESCREEN, 
            decks: Vec::new(),
            logger: Logger::new(),
        };
    }
}

impl App for Manki {
    
    fn name(&self) -> &str {
        return "Manki";
    }

    fn update(&mut self, ctx: &Context, _frame: &Frame<>) {
        CentralPanel::default().show(ctx, |ui|{
            match &self.state {
                State::HOMESCREEN => {
                    render_homescreen(ui, &mut self.logger);
                }
                State::STUDYSCREEN => {

                }
            }
        });
    }
}

fn render_homescreen(ui: &mut Ui, mut logger: &mut Logger) {
    let files =  match file_manager::list_files(file_manager::decks_directory()) {
        Ok(f) => f,
        Err(_) => Vec::new(),
    };

    let mut decks: Vec<Deck> = files.iter().map(|f| {
        return match Deck::read_from(f, &mut logger) {
            Ok(d) => d,
            Err(_) => Deck::empty("Failed to Load"),
        }
    }).collect();

    for i in 0..20 {
        decks.push(Deck::empty(&i.to_string()));
    }

    for deck in decks {
        ui.horizontal( |ui| {
            ui.add_sized([100., 5.], Button::new(deck.title));
        });
    }
}

fn main() {
    let app = Manki::default();
    let options = NativeOptions::default();
    run_native(Box::new(app), options);
}
