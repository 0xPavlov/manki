mod deck;
mod file_manager;
mod gui;
mod icons;
mod logger;

use crate::deck::Deck;
use crate::logger::Logger;
use eframe::{
    egui::Context,
    epi::{App, Frame},
    run_native, NativeOptions,
};

pub enum State {
    HOMESCREEN,
    STUDYSCREEN,
}

struct Manki {
    state: State,
    curr_deck: Deck, //current deck, either the one currently being studied, edited or created
    logger: Logger,
}

impl Manki {
    fn default() -> Manki {
        return Manki {
            state: State::HOMESCREEN,
            curr_deck: Deck::empty("Empty"),
            logger: Logger::new(),
        };
    }
}

impl App for Manki {
    fn name(&self) -> &str {
        return "Manki";
    }

    fn update(&mut self, ctx: &Context, frame: &Frame) {
        frame.drag_window();

        match &self.state {
            State::HOMESCREEN => gui::render_homescreen(ctx, &mut self.logger),
            State::STUDYSCREEN => {}
        }
    }
}

fn main() {
    let app = Manki::default();
    let options = NativeOptions::default();
    run_native(Box::new(app), options);
}
