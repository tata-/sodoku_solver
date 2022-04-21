use crate::model::model::Grid;

#[path = "model/model.rs"] mod model;

fn main() {
    println!("Hello, world!");

    let g = Grid::new();
}
