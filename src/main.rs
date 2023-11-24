mod deck;
mod gui;
mod io_manager;
mod serde_util;
mod settings;

use crate::deck::Deck;
use crate::settings::Settings;
use eframe::{run_native, App, CreationContext, Frame, NativeOptions};
use egui::Context;
use io_manager::check_for_preferences;

pub enum State {
    HOMESCREEN,
    STUDYSCREEN,
    EDITSCREEN,
    INSTALLATION,
}

struct Manki {
    pub(crate) state: State,
    curr_deck: Deck, //current deck, either the one currently being studied, edited or created
    index: usize,

    window_width: f32,
    window_height: f32,
    pub(crate) settings: Settings,
}

impl Manki {
    fn default(_cc: &CreationContext<'_>) -> Manki {
        let mut app = Manki {
            state: State::INSTALLATION,
            curr_deck: Deck::empty("Empty").as_unserializable(),
            index: 0,
            window_height: 0.,
            window_width: 0.,
            settings: Settings::new(),
        };

        match check_for_preferences() {
            Ok(entry) => {
                app.settings.app_directory = entry.app_path;
                app.settings
                    .save_json_to_file(app.settings.app_directory())
                    .unwrap();
                app.state = State::HOMESCREEN;
            }
            Err(_) => {}
        }
        app
    }

    fn reset(&mut self) {
        match self.state {
            State::STUDYSCREEN => {
                self.index = 0;
                self.curr_deck
                    .save_to_json(self.settings.decks_directory())
                    .unwrap_or_else(|err| {});
                self.state = State::HOMESCREEN;
            }
            _ => {
                unimplemented!("Lol")
            }
        }
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
            State::INSTALLATION => gui::render_installation_wizard(ctx, self),
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
