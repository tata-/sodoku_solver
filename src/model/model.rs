pub mod model {

    use std::cell::Cell;
    use std::fmt::{Debug, Formatter};

    #[derive(Copy, Clone)]
    pub struct Place {
        pub value: Option<usize>,
        pub possible: [bool; 9],
    }

    pub struct Line {
        pub places: Vec<Cell<Place>>,
    }

    pub struct Block {
        pub places: Vec<Cell<Place>>,
    }

    pub struct Grid {
        pub places: Vec<Cell<Place>>,
        pub rows: Vec<Line>,
        pub columns: Vec<Line>,
        pub blocks: Vec<Block>,
    }

    impl Line {
        pub fn new() -> Line {
            let places = Vec::new();
            Line { places }
        }

        pub fn add_place(&mut self, p: Cell<Place>) {
            self.places.push(p);
        }

        pub fn pretty(&self) {
            for p in self.places.iter() {
                p.get().pretty();
            }
            println!("|");
        }
    }

    impl Block {
        pub fn new() -> Block {
            let places = Vec::new();
            Block { places }
        }

        pub fn add_place(&mut self, p: Cell<Place>) {
            self.places.push(p);
        }
    }

    impl Place {
        pub fn new(value: usize) -> Place {
            let mut o = None;

            let mut pos = [false; 9];

            if value > 0 && value < 10 {
                o = Some(value);
                pos[value-1] = true;
            } else {
                pos = [true; 9];
            }

            Place {
                value: o,
                possible: pos,
            }
        }

        pub fn pretty(&self) {
            print!("|");
            match self.value {
                None => print!(" "),
                Some(v) => print!("{}", v),
            }
        }

        pub fn count_possibles(&self) -> usize {
            let mut ret = 0;
            for b in self.possible {
                if b {
                    ret += 1;
                }
            }
            return ret;
        }

        pub fn set_from_possibles(&mut self) {
            let mut i = 1;
            for b in self.possible {
                if b {
                    if self.value == None {
                        self.value = Some(i);
                    } else {
                        panic!("Value is already set");
                    }
                }
                i += 1;
            }
        }
    }

    impl Debug for Place {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("Place")
                .field("value:", &self.value)
                .finish()
        }
    }

    impl Grid {
        pub fn new(value_str: &str) -> Grid {
            let values: Vec<&str> = value_str.split(",").collect();

            if values.len() != 9 * 9 {
                panic!("Not enough number for a Sodoku puzzle {}", values.len());
            }

            let mut places = Vec::new();
            let mut rows = Vec::new();
            let mut columns = Vec::new();
            let mut blocks = Vec::new();

            for _x in 0..9 {
                rows.push(Line::new());
                columns.push(Line::new());
                blocks.push(Block::new());
            }

            let mut index = 0;
            for s in values {
                let i = s.parse::<usize>().unwrap();
                let place = Place::new(i);

                let x = index % 9;
                let y = index / 9;
                let b = Grid::block_index(index);

                rows.get_mut(y).unwrap().add_place(Cell::new(place));
                columns.get_mut(x).unwrap().add_place(Cell::new(place));
                blocks.get_mut(b).unwrap().add_place(Cell::new(place));
                places.push(Cell::new(place));
                index += 1;
            }

            Grid {
                places,
                rows,
                columns,
                blocks,
            }
        }

        pub fn block_index(index: usize) -> usize {
            return (index % 9) / 3 + 3 * (index / 27);
        }

        pub fn pretty(&self) {
            for row in self.rows.iter() {
                row.pretty();
            }
        }

        pub fn solve(&mut self) {
            for p in self.places.iter_mut() {
                if p.get().count_possibles() == 1 {
                    p.get_mut().set_from_possibles();
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::model::model::Grid;
    use std::borrow::Borrow;

    #[test]
    fn dummy_test() {
        println!("It worked");
    }

    #[test]
    #[should_panic]
    fn test_zero_length() {
        let _g = Grid::new("");
    }

    #[test]
    #[should_panic]
    fn test_too_long() {
        let mut s = generate_empty_line();
        s.push_str(",0");
        let _g = Grid::new(s.borrow());
    }

    #[test]
    #[should_panic]
    fn test_two_length() {
        let _g = Grid::new("1,2");
    }

    #[test]
    fn test_sizes() {
        let g = Grid::new(&generate_empty_line());
        assert_eq!(g.places.len(), 81);
        assert_eq!(g.columns.len(), 9);

        for col in g.columns.iter() {
            assert_eq!(col.places.len(), 9);
        }

        assert_eq!(g.blocks.len(), 9);

        for block in g.blocks.iter() {
            assert_eq!(block.places.len(), 9);
        }

        assert_eq!(g.rows.len(), 9);

        for row in g.rows.iter() {
            assert_eq!(row.places.len(), 9);
        }
    }

    #[test]
    fn test_first_item() {
        let g = Grid::new(&generate_empty_line());
        let p = g.places.get(0).unwrap();
        p.get().pretty();
    }

    #[test]
    fn test_with_easy() {
        let grid = Grid::new(&generate_4338937849());
        let p = grid.places.get(0).unwrap();

        assert_eq!(p.get().value.unwrap(), 1);
        grid.pretty();

        for p in grid.places.iter() {
            match p.get().value {
                None => assert_eq!(p.get().count_possibles(), 9),
                Some(_) => assert_eq!(p.get().count_possibles(), 1),
            }
        }
    }

    #[test]
    fn sand_box() {
        let s = "1, 2, 3, 0, 9";
        let v = s.split(", ");
        for i in v {
            println!("{}", i);
        }
    }
    #[test]
    fn test_block_indexing() {
        assert_eq!(Grid::block_index(0), 0);
        assert_eq!(Grid::block_index(1), 0);
        assert_eq!(Grid::block_index(2), 0);
        assert_eq!(Grid::block_index(3), 1);
        assert_eq!(Grid::block_index(6), 2);

        assert_eq!(Grid::block_index(9), 0);
        assert_eq!(Grid::block_index(14), 1);
        assert_eq!(Grid::block_index(19), 0);
        assert_eq!(Grid::block_index(26), 2);
        assert_eq!(Grid::block_index(27), 3);
        assert_eq!(Grid::block_index(80), 8);
    }

    fn generate_empty_line() -> String {
        let zero_fn = || return 0;
        return generate_line(zero_fn);
    }

    fn generate_4338937849() -> String {
        let s = "1,5,2,0,0,6,8,0,0,\
                     0,6,9,0,0,0,0,0,0,\
                     3,7,0,0,1,2,0,9,0,\
                     7,2,0,0,0,1,4,0,0,\
                     0,0,0,7,0,4,0,0,0,\
                     0,0,6,2,0,0,0,7,9,\
                     0,1,0,5,8,0,0,6,7,\
                     0,0,0,0,0,0,3,1,0,\
                     0,0,3,1,0,0,5,8,2"
            .to_string();
        return s;
    }

    fn generate_line(f: fn() -> i32) -> String {
        let mut ret_val = String::new();

        ret_val.push_str(&*f().to_string());

        for _i in 1..9 * 9 {
            ret_val.push_str(",");
            ret_val.push_str(&*f().to_string());
        }
        return ret_val.to_owned();
    }
}
