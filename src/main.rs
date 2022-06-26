
use eframe::egui;
use eframe::egui::Context;
use eframe::epi::Frame;
use std::borrow::Borrow;

#[path = "model/model.rs"] mod model;

fn main() {
    println!("Hello, world!");
    build_ui();

}

fn build_ui(){
    let options  = eframe::NativeOptions::default();
    eframe::run_native( Box::new( Box::new(SolverApp::default())), options);
}

struct SolverApp {
    name: String,
}

impl Default for SolverApp{
    fn default() -> Self {
        Self {
            name: "Name".to_owned(),
        }
    }
}

impl eframe::epi::App for Box<SolverApp>{
    fn update(&mut self, ctx: &Context, _frame: &Frame) {

        egui::CentralPanel::default().show(ctx, |ui| {

            ui.heading("Sodoku Solver App");

            ui.horizontal( |ui| {
                ui.label("Your name: ");
                ui.text_edit_singleline(&mut self.name);
            });

            if ui.button("Reset Name").clicked(){
                self.name="Sandor".to_owned();
            }

            ui.label(format!("Hello '{} !!' ", self.name));

            let texture = ctx.load_texture("Test image", egui::ColorImage::example());

            ui.image(texture.id(), texture.size_vec2());
        });



    }

    fn name(&self) -> &str {
        return self.name.borrow();
    }
}