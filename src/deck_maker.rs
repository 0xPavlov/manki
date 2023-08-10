use eframe::{
    egui::{
        CentralPanel,
        Context,
        TextEdit,
        Ui,
        Button,
    },
    epi::{
        App,
        Frame,
    },
    NativeOptions,
    run_native,
};

impl Test { 
    fn from(lgr: Logger) -> Test {
        return Test { amount: 0, arr: Vec::new()};
    }


impl App for Test {

    fn name(&self) -> &str {
        return "Test";
    }

    fn update(&mut self, ctx: &Context, _frame: &Frame<>) { 
        CentralPanel::default().show(ctx, |ui| {
            ui.heading("Deck Maker");
            ui.horizontal( |ui| {
                    if ui.button("Add a new Card!").clicked() {
                        self.amount += 1;
                        self.arr.push(String::new());
                    }
                }
            );


            for i in 0..self.amount {
                ui.horizontal(|ui| {
                    ui.label(format!("card {}", i + 1));
                    
                    ui.add(
                       TextEdit::singleline(&mut self.arr[i]), 
                    );

                    ui.add(
                       TextEdit::singleline(&mut self.arr[i]), 
                    );

                    if ui.button("🗑").clicked() {
                    };
                }); 
            }

            let _ = egui::Slider::new(&mut self.amount, 0..=15).text("amount of cards");

        });
    }
}

