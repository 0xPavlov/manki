use crate::{
    deck::Evaluation,
    file_manager::{decks_directory, list_files},
    gui_util::DeckButton,
    Deck, Manki, State,
};
use eframe::egui::{Button, CentralPanel, Context};
use eframe::egui::{RichText, TopBottomPanel};
use egui::Key;

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
                    app.curr_deck.sort();
                    app.state = State::STUDYSCREEN;
                }
            });
        }
    });
}

pub(crate) fn render_studyscreen(ctx: &Context, app: &mut Manki) {
    let curr_card_opt = app.curr_deck.get(app.index);

    if curr_card_opt.is_none() {
        app.index = 0;
        let _ = app.curr_deck.save_to_json();
        app.state = State::HOMESCREEN;
        return;
    }
    let curr_card = curr_card_opt.unwrap();

    ctx.input(|input| {
        if input.key_pressed(Key::Space) {
            curr_card.flip();
        }
    });

    CentralPanel::default().show(ctx, |ui| {
        ui.vertical_centered(|ui| {
            ui.heading(RichText::new(curr_card.display_text()).size(30.));
        });
    });

    TopBottomPanel::bottom("bootom_panel").show(ctx, |ui| {
        //horizontal centering is quite hard in eframe:.egui so this is a workaround

        ui.columns(4, |columns| {
            columns[0].vertical_centered(|ui| {
                if ui.add_sized([200., 5.], Button::new("Very Bad")).clicked() {
                    curr_card.update_eval(Evaluation::VeryBad);
                    app.index += 1;
                }
            });

            columns[1].vertical_centered(|ui| {
                if ui.add_sized([200., 5.], Button::new("Bad")).clicked() {
                    curr_card.update_eval(Evaluation::Bad);
                    app.index += 1;
                }
            });

            columns[2].vertical_centered(|ui| {
                if ui.add_sized([200., 5.], Button::new("Good")).clicked() {
                    curr_card.update_eval(Evaluation::Good);
                    app.index += 1;
                }
            });

            columns[3].vertical_centered(|ui| {
                if ui.add_sized([200., 5.], Button::new("Very Good")).clicked() {
                    curr_card.update_eval(Evaluation::VeryGood);
                    app.index += 1;
                }
            });
        });
    });
}
