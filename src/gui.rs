use crate::{
    deck::{Card, Deck},
    file_manager::{decks_directory, list_files},
    gui_util::DeckButton,
    Manki, State,
};
use eframe::egui::{CentralPanel, Context};
use eframe::egui::{Key, RichText, TopBottomPanel};

pub(crate) fn render_homescreen(ctx: &Context, app: &mut Manki) {
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

        let decks: Vec<DeckButton> = DeckButton::paths_to_buttons(files);

        for deck in decks {
            ui.horizontal(|ui| {
                if ui.add_sized([500., 5.], deck.button).clicked() {
                    app.curr_deck = Deck::read_from(&deck.path).unwrap();
                    app.state = State::STUDYSCREEN;
                }
            });
        }
    });
}

pub(crate) fn render_studyscreen(ctx: &Context, app: &mut Manki) {
    let curr_card = app.curr_deck.get(app.index);

    if ctx.input().key_pressed(Key::Space) {
        curr_card.flip();
    }

    CentralPanel::default().show(ctx, |ui| {
        ui.vertical_centered(|ui| {
            ui.heading(RichText::new(curr_card.display_text()).size(15.));
        });
    });
}
