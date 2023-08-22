use crate::{
    file_manager::{decks_directory, file_name, list_files},
    Manki,
};
use eframe::egui::TopBottomPanel;
use eframe::egui::{Button, CentralPanel, Context};
use std::path::PathBuf;

pub(crate) fn render_homescreen(ctx: &Context, _app: &mut Manki) {
    TopBottomPanel::top("top_panel").show(ctx, |ui| {
        ui.label("Lorem ipsum dolor sit amet, qui minim labore adipisicing minim sint cillum sint consectetur cupidatat.");
        ui.label("Lorem ipsum dolor sit amet, qui minim labore adipisicing minim sint cillum sint consectetur cupidatat.");
        ui.label("Lorem ipsum dolor sit amet, qui minim labore adipisicing minim sint cillum sint consectetur cupidatat.");
        ui.label("Lorem ipsum dolor sit amet, qui minim labore adipisicing minim sint cillum sint consectetur cupidatat.");
    });

    CentralPanel::default().show(ctx, |ui| {
        let files = match list_files(decks_directory()) {
            Ok(f) => f,
            Err(_) => Vec::new(),
        };

        let mut decks: Vec<DeckButton> = Vec::new();

        for i in 0..files.len() {
            let curr_path: &PathBuf = &files[i];
            let display: &str = &file_name(&curr_path);
            decks.push(DeckButton::from(
                display.to_string(),
                curr_path.to_path_buf(),
                Button::new(display),
            ));
        }

        for deck in decks {
            ui.horizontal(|ui| {
                if ui.add_sized([500., 5.], deck.button).clicked() {
                    println!("{}", deck.path.to_str().unwrap());
                }
            });
        }
    });
}

struct DeckButton {
    //the display name of the button, visible to the user
    display_name: String,
    //the absolute path to the deck this button is representing
    path: PathBuf,
    button: Button,
}

impl DeckButton {
    pub(crate) fn from(name: String, pth: PathBuf, bttn: Button) -> Self {
        DeckButton {
            display_name: name,
            path: pth,
            button: bttn,
        }
    }
}
