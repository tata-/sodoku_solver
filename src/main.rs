
use std::borrow::{Borrow, BorrowMut};

use opencv::prelude::VideoCaptureTrait;
use opencv::videoio;
use opencv::core::{Mat, Vector, MatTraitConstManual, CV_8UC4, CV_8UC3, ToOutputArray};
use opencv::videoio::VideoCapture;
use opencv::imgcodecs;
use opencv::prelude::MatTraitManual;

use opencv::imgproc::{cvt_color, COLOR_BGR2RGBA};
use std::ops::Mul;
use std::time::{SystemTime, UNIX_EPOCH};
use opencv::highgui::{named_window, WINDOW_AUTOSIZE, wait_key, imshow, destroy_all_windows};

#[path = "model/model.rs"] mod model;



fn main() {
    println!("Hello, world!");
    let mut app = SolverApp::default();
    app.build_ui();

}



struct SolverApp {
    name: String,
    cam: VideoCapture,
    frame: Mat,
}

impl SolverApp{
    fn build_ui(&mut self){
        named_window("Camera", WINDOW_AUTOSIZE);
        while (wait_key(33).unwrap() != 27) {
            self.cam.read(&mut self.frame);
            imshow("Camera", &self.frame);
        }

        destroy_all_windows();
    }
}


impl Default for SolverApp{
    fn default() -> Self {
        Self {
            name: "Name".to_owned(),
            cam : videoio::VideoCapture::new(0, videoio::CAP_ANY).unwrap(),
            frame : Mat::default(),
        }
    }
}

