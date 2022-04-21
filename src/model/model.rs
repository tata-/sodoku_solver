
pub mod model {
    use std::rc::Rc;
    use std::fmt::{Debug};

    #[derive(Debug)]
    #[derive(PartialEq)]
    pub struct Place {
        pub value: Option<usize>
    }

    pub struct Line {
        pub places: Vec<Rc<Place>>,
    }

    pub struct Block {
        pub places: Vec<Rc<Place>>,
    }

    pub struct Grid {
        pub places: Vec<Rc<Place>>,
        pub rows: Vec<Line>,
        pub columns: Vec<Line>,
        pub blocks: Vec<Block>,
    }

    impl Line {
        pub fn new() -> Line {
            let places = Vec::new();
            Line { places }
        }

        pub fn add_place(&mut self, p: Rc<Place>) {
            self.places.push(p);
        }
    }

    impl Block {
        pub fn new() -> Block {
            let places = Vec::new();
            Block { places }
        }
    }

    impl Place {
        pub fn new(value: usize) -> Place {

            let mut o = None;
            if value >0 && value < 10{
                o = Some(value);
            }

            Place { value: o}
        }
    }

    impl Grid {
        pub fn new() -> Grid {
            let mut places = Vec::new();
            let mut rows = Vec::new();
            let mut columns = Vec::new();
            let mut blocks = Vec::new();

            for _x in 0..9 {
                rows.push(Line::new());
                columns.push(Line::new());
                blocks.push(Block::new());
            }

            for i in 0..9 * 9 {
                let place = Rc::new( Place::new(i));

                let x = i % 9;
                let y = i / 9;
                rows.get_mut(y).unwrap().add_place(place.clone());
                columns.get_mut(x).unwrap().add_place(place.clone());
                places.push(place.clone());
            }


            Grid {
                places,
                rows,
                columns,
                blocks,
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::model::model::Grid;

    #[test]
    fn dummy_test() {
        println!("It worked");
    }

    #[test]
    fn test_sizes() {
        let g = Grid::new();
        assert_eq!(g.places.len(), 81);
        assert_eq!(g.columns.len(), 9);
        assert_eq!(g.blocks.len(), 9);
        assert_eq!(g.rows.len(), 9);
    }

    #[test]
    fn test_first_item() {
        let g = Grid::new();
        let p = g.places.get(0).unwrap();
        assert_eq!(*p, *g.rows.get(0).unwrap().places.get(0).unwrap());
        assert_ne!(*p, *g.rows.get(0).unwrap().places.get(1).unwrap());
    }
}