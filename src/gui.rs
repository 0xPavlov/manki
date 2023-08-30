use crate::{
    deck::Evaluation,
    file_manager::{decks_directory, list_files},
    gui_util::WidgetWrapper,
    Deck, Manki, State,
};
use eframe::egui::TopBottomPanel;
use eframe::egui::{Button, CentralPanel, Context};
use egui::{DragValue, Key, Label, ScrollArea};

pub(crate) fn render_homescreen(ctx: &Context, app: &mut Manki) {
    TopBottomPanel::top("top_panel").show(ctx, |ui| {
        ui.horizontal(|ui| {
            if ui.button("New Deck").clicked() {
                app.state = State::EDITSCREEN;
                app.curr_deck = Deck::empty("");
            }
        });
    });

    CentralPanel::default().show(ctx, |ui| {
        let files = match list_files(decks_directory()) {
            Ok(f) => f,
            Err(e) => {
                app.logger.log_error(e.to_string());
                Vec::new()
            }
        };

        let decks: Vec<Deck> = files
            .iter()
            .map(|path| match Deck::read_from(path) {
                Ok(deck) => deck,
                Err(err) => {
                    app.logger.log_error(err.to_string());

                    Deck::empty("Failed to Load Deck")
                }
            })
            .collect();

        ScrollArea::vertical()
            .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysHidden)
            .show(ui, |ui| {
                for deck in decks {
                    ui.horizontal(|ui| {
                        if ui
                            .add_sized([app.window_width, 5.], Button::new(&deck.title))
                            .clicked()
                        {
                            app.curr_deck = deck;
                            app.curr_deck.sort();
                            app.state = State::STUDYSCREEN;
                        }
                    });
                }
            });
    });

    TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
        ui.hyperlink_to("GitHub", "https://github.com/0xPavlov/manki");
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

    TopBottomPanel::top("top_panel").show(ctx, |ui| {
        ui.vertical_centered(|ui| {
            ui.add(curr_card.heading());
        });
    });

    CentralPanel::default().show(ctx, |ui| {
        for widget in curr_card.body() {
            match widget {
                WidgetWrapper::Label(label_text) => ui.add(Label::new(label_text)),
                WidgetWrapper::Image(_image_path) => unimplemented!(),
            };
        }
    });

    TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
        //horizontal centering is quite hard in egui so this is a workaround

        let width = app.window_width;
        let padding = 50.;
        let button_amount = 4.;
        let button_width = (width - (padding * (button_amount + 1.))) / button_amount;
        let button_height = 30.;

        ui.columns(4, |columns| {
            columns[0].vertical_centered(|ui| {
                if ui
                    .add_sized([button_width, button_height], Button::new("Very Bad"))
                    .clicked()
                {
                    curr_card.update_eval(Evaluation::VeryBad);
                    app.index += 1;
                }
            });

            columns[1].vertical_centered(|ui| {
                if ui
                    .add_sized([button_width, button_height], Button::new("Bad"))
                    .clicked()
                {
                    curr_card.update_eval(Evaluation::Bad);
                    app.index += 1;
                }
            });

            columns[2].vertical_centered(|ui| {
                if ui
                    .add_sized([button_width, button_height], Button::new("Good"))
                    .clicked()
                {
                    curr_card.update_eval(Evaluation::Good);
                    app.index += 1;
                }
            });

            columns[3].vertical_centered(|ui| {
                if ui
                    .add_sized([button_width, button_height], Button::new("Very Good"))
                    .clicked()
                {
                    curr_card.update_eval(Evaluation::VeryGood);
                    app.index += 1;
                }
            });
        });
    });
}
