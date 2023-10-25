use crate::settings::Settings;
use crate::{
    deck::Evaluation, gui_util::WidgetWrapper, io_manager::list_files, Deck, Manki, State,
};
use eframe::egui::TopBottomPanel;
use eframe::egui::{Button, CentralPanel, Context};
use egui::{Align, Image, Key, Label, Layout, ScrollArea, Vec2};

pub(crate) fn render_homescreen(ctx: &Context, app: &mut Manki) {
    TopBottomPanel::top("top_panel").show(ctx, |ui| {
        ui.with_layout(Layout::left_to_right(Align::TOP), |ui| {
            ui.heading("Lorem ipsum dolor sit amet, qui minim labore adipisicing minim sint cillum sint consectetur cupidatat.");
        });
    });

    CentralPanel::default().show(ctx, |ui| {
        let files = match list_files(app.settings.decks_directory()) {
            Ok(f) => f,
            Err(err) => {
                eprintln!("{}", err.to_string());
                Vec::new()
            }
        };

        let decks: Vec<Deck> = files
            .iter()
            .map(|path| match Deck::read_from(path) {
                Ok(deck) => deck,
                Err(err) => {
                    eprintln!(
                        "{}",
                        format!(
                            "Deserialisation of Deck {} failed due to {}",
                            path.to_str().unwrap(),
                            err.to_string()
                        )
                    );

                    Deck::empty("Failed to deserialise Deck").as_unserializable()
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
        app.curr_deck
            .save_to_json(app.settings.decks_directory())
            .unwrap_or_else(|err| {
                eprintln!(
                    "{}",
                    format!(
                        "Saving Deck {} failed due to {}",
                        app.curr_deck.title,
                        err.to_string()
                    )
                );
            });
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
            ui.label(curr_card.heading());
        });
    });

    CentralPanel::default().show(ctx, |ui| {
        for widget in curr_card.body() {
            match widget {
                WidgetWrapper::Label(label_text) => ui.add(Label::new(label_text)),
                WidgetWrapper::Image(image_path) => ui.add(Image::new(image_path)),
            };
        }
    });

    TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
        //Padding is as follows
        // 25% side padding - 2% item padding - 14% button width - 2% item padding...

        let width = app.window_width;
        let side_padding = 0.25 * width;
        let item_padding = 0.02 * width;
        let button_width = (0.42 * width) / 3.;
        let button_height = 30.;
        let evals = vec![Evaluation::Again, Evaluation::Hard, Evaluation::Easy];

        ui.horizontal(|ui| {
            // remove any preset padding
            ui.style_mut().spacing.item_spacing = Vec2::new(item_padding, 0.);

            ui.add_space(side_padding);

            for eval in evals {
                if ui
                    .add_sized([button_width, button_height], Button::new(eval.to_string()))
                    .clicked()
                {
                    curr_card.update_eval(eval);
                    app.index += 1;
                }
            }

            ui.add_space(side_padding);
        });
    });
}

pub(crate) fn render_installation_wizard(ctx: &Context, app: &mut Manki) {
    TopBottomPanel::top("top_panel").show(ctx, |ui| {
        ui.heading("Manki Installation-Wizard");
    });

    CentralPanel::default().show(ctx, |ui| {
        ui.label("Please enter the absolute path to the directory Manki should store its data at");
        ui.text_edit_singleline(&mut app.settings.app_directory);
    });

    TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
        ui.with_layout(
            Layout::centered_and_justified(egui::Direction::TopDown),
            |ui| {
                if ui.button("Done").clicked() {
                    Settings::register_preferences(&app.settings.app_directory).unwrap();
                    app.state = State::HOMESCREEN;
                }
            },
        );
    });
}
