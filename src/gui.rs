use crate::deck::Deck;
use crate::file_manager;
use crate::logger::Logger;
use eframe::egui::{Button, Ui};

pub(crate) fn render_homescreen(ui: &mut Ui, _logger: &mut Logger) {
    let files = match file_manager::list_files(file_manager::decks_directory()) {
        Ok(f) => f,
        Err(_) => Vec::new(),
    };

    let decks: Vec<Deck> = files
        .iter()
        .map(|f| {
            return match Deck::read_from(f) {
                Ok(d) => d,
                Err(_) => Deck::empty("Failed to Load"),
            };
        })
        .collect();

    for deck in decks {
        ui.horizontal(|ui| {
            ui.add_sized([100., 5.], Button::new(deck.title));
        });
    }
}
