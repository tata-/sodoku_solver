
use eframe::egui;
use eframe::egui::Context;
use eframe::epi::Frame;
use std::borrow::{Borrow, BorrowMut};

use opencv::prelude::VideoCaptureTrait;
use opencv::videoio;
use opencv::core::{Mat, Vector, MatTraitConstManual, CV_8UC4, CV_8UC3, ToOutputArray};
use opencv::videoio::VideoCapture;
use opencv::imgcodecs;
use opencv::prelude::MatTraitManual;

use egui::{ColorImage, Color32};
use opencv::imgproc::{cvt_color, COLOR_BGR2RGBA};
use std::ops::Mul;
use std::time::{SystemTime, UNIX_EPOCH};

#[path = "model/model.rs"] mod model;



fn main() {
    println!("Hello, world!");
    build_ui();

}

fn build_ui(){
    let options  = eframe::NativeOptions::default();
    eframe::run_native( Box::new( Box::new(SolverApp::default())), options);
}

fn getNow() -> u128 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis()
}

struct SolverApp {
    name: String,
    cam: VideoCapture,
    frame: Mat,
    rgba_frame: Mat,
}

impl Default for SolverApp{
    fn default() -> Self {
        Self {
            name: "Name".to_owned(),
            cam : videoio::VideoCapture::new(0, videoio::CAP_ANY).unwrap(),
            frame : Mat::default(),
            rgba_frame: Mat::default(),
        }
    }
}

impl eframe::epi::App for Box<SolverApp>{
    fn update(&mut self, ctx: &Context, _frame: &Frame) {

        egui::CentralPanel::default().show(ctx, |ui| {

            let start = getNow();

            ui.heading("Sodoku Solver App");

            ui.horizontal( |ui| {
                ui.label("Your name: ");
                ui.text_edit_singleline(&mut self.name);
            });

            if ui.button("Reset Name").clicked(){
                self.name="Sandor".to_owned();
            }

            ui.label(format!("Hello '{} !!' ", self.name));

            println!("Before read {:?}", getNow() - start);
            self.cam.read(&mut self.frame);
            println!("After read {:?}", getNow() - start);


            let size = self.frame.size().unwrap();

            let w =  size.width as usize;
            let h = size.height as usize;



            cvt_color(&self.frame, self.rgba_frame.borrow_mut(), COLOR_BGR2RGBA, 4);
            println!("transform done {:?}", getNow()-start);

            let img=ColorImage::from_rgba_unmultiplied([w,h], self.rgba_frame.data_bytes().unwrap());
            println!("img done {:?}", getNow() - start);

            let texture = ctx.load_texture("Test image", img);
            ui.image(texture.id(), texture.size_vec2());
            println!("ui done {:?}", getNow() - start);

        });



    }

    fn name(&self) -> &str {
        return self.name.borrow();
    }



}

