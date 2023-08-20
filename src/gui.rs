use crate::deck::Deck;
use crate::file_manager;
use crate::logger::Logger;
use eframe::egui::TopBottomPanel;
use eframe::egui::{Button, CentralPanel, Context};

pub(crate) fn render_homescreen(ctx: &Context, _logger: &mut Logger) {
    TopBottomPanel::top("top_panel").show(ctx, |ui| {
        ui.label("TOPTEXT");
    });

    CentralPanel::default().show(ctx, |ui| {
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
                ui.add_sized([500., 5.], Button::new(deck.title));
            });
        }
    });
}
