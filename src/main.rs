mod deck;
mod file_manager;
mod gui;
mod gui_util;
mod icons;
mod logger;
mod serde_util;

use crate::deck::Deck;
use crate::logger::Logger;
use eframe::{run_native, App, CreationContext, Frame, NativeOptions};
use egui::Context;

pub enum State {
    HOMESCREEN,
    STUDYSCREEN,
    EDITSCREEN,
}

struct Manki {
    state: State,
    curr_deck: Deck, //current deck, either the one currently being studied, edited or created
    index: usize,
    window_width: f32,
    window_height: f32,
    _logger: Logger,
}

impl Manki {
    fn default(_cc: &CreationContext<'_>) -> Manki {
        return Manki {
            state: State::HOMESCREEN,
            curr_deck: Deck::empty("Empty"),
            index: 0,
            window_height: 0.,
            window_width: 0.,
            _logger: Logger::new(),
        };
    }
}

impl App for Manki {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        frame.drag_window();
        let window_size = frame.info().window_info.size;
        self.window_width = window_size.x;
        self.window_height = window_size.y;

        match &self.state {
            State::HOMESCREEN => gui::render_homescreen(ctx, self),
            State::STUDYSCREEN => gui::render_studyscreen(ctx, self),
            State::EDITSCREEN => {}
        }
    }
}

fn main() -> Result<(), eframe::Error> {
    let native_options = NativeOptions::default();
    run_native(
        "Manki",
        native_options,
        Box::new(|cc| Box::new(Manki::default(cc))),
    )
}
