use std::ffi::OsStr;
use std::path::PathBuf;

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

        let decks: Vec<(&str, &PathBuf)> = files
            .iter()
            .map(|f| {
                (
                    f.file_stem()
                        .unwrap_or(OsStr::new("Invaild File"))
                        .to_str()
                        .unwrap(),
                    f,
                )
            })
            .collect();

        for deck in decks {
            ui.horizontal(|ui| {
                let button = Button::new(deck.0);
                if ui.add_sized([500., 5.], button).clicked() {}
            });
        }
    });
}
