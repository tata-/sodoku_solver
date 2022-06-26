pub mod model {
    use std::cell::Cell;
    use std::fmt::{Debug, Formatter};
    use std::ops::Add;
    use std::slice::{Iter, IterMut};
    use std::sync::{Arc, Mutex};

    use array_init::array_init;

    #[derive(Copy, Clone)]
    pub struct Place {
        pub value: Option<usize>,
        pub possible: [bool; 9],
    }

    pub struct Line {
        pub places: [Arc<Mutex<Place>>; 9],
        initialized: usize,
    }

    pub struct Grid {
        pub places: Vec<Arc<Mutex<Place>>>,
        pub rows: [Line; 9],
        pub columns: Vec<Line>,
        pub blocks: Vec<Line>,
    }

    impl Line {
        pub fn new() -> Line {
            let places = array_init(|_i| Arc::new(Mutex::new(Place::new(0))));
            Line { places: places, initialized: 0 }
        }

        pub fn add_place(&mut self, p: Arc<Mutex<Place>>) {
            self.places[self.initialized] = p;
            self.initialized += 1;
        }

        pub fn pretty(&self) -> String {
            let mut s = "".to_owned();
            for p in self.places.iter() {
                s = s.add(&p.lock().unwrap().pretty());
            }
            s = s.add("|").add("\n");
            return s;
        }
    }

    impl Place {
        pub fn new(value: usize) -> Place {
            let mut o = None;

            let mut pos = [false; 9];

            if value > 0 && value < 10 {
                o = Some(value);
                pos[value - 1] = true;
            } else {
                pos = [true; 9];
            }

            Place {
                value: o,
                possible: pos,
            }
        }

        pub fn pretty(&self) -> String {
            let mut str = "|".to_owned();
            match self.value {
                None => str = str + " ",
                Some(v) => str = str + &v.to_string(),
            }
            return str;
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

            println!("Possibles are {:?}", self.possible);

            for b in self.possible {
                if b {
                    println!("Value found it is {}", i);
                    if self.value == None {
                        self.value = Some(i);
                        println!("Value set as {:?}", self.value);
                    } else {
                        panic!("Value is already set");
                    }
                }
                i += 1;
            }
        }

        pub fn update(&mut self, found: Iter<usize>) -> bool {
            if self.value != None {
                return false;
            }
            let mut ret = false;
            for &index in found {
                if self.possible[index - 1] {
                    ret = true;
                    self.possible[index - 1] = false;
                }
            }
            println!("    possibles are {:?}", self.possible);
            return ret;
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
            let mut rows: [Line; 9] = array_init(|_| Line::new());
            let mut columns = Vec::new();
            let mut blocks = Vec::new();

            for _x in 0..9 {
                // rows.push(Line::new());
                columns.push(Line::new());
                blocks.push(Line::new());
            }

            let mut index = 0;
            for s in values {
                let i = s.parse::<usize>().unwrap();
                let place = Place::new(i);

                let x = index % 9;
                let y = index / 9;
                let b = Grid::block_index(index);
                let arc = Arc::new(Mutex::new(place));

                Cell::new(place);
                //rows.get_mut(y).unwrap().add_place(Cell::new(place));
                columns.get_mut(x).unwrap().add_place(Arc::clone(&arc));
                blocks.get_mut(b).unwrap().add_place(Arc::clone(&arc));
                places.push(Arc::clone(&arc));
                rows[y].places[x] = arc;
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

        pub fn pretty(&self) -> String {
            let mut s = "".to_owned();
            for row in self.rows.iter() {
                let rs = row.pretty();
                s = s.add(&rs)
            }
            print!("Grid pretty is: \n{}", s);
            return s;
        }

        pub fn solve(&mut self) {

            let mut solved = false;
            let mut update = true;
            let mut cycles = 0;
            while !solved {
                while update {
                    self.check_places();
                    update_line(self.rows.iter_mut());
                    print!("Rows updated");
                    update_line(self.columns.iter_mut());
                    print!("Columns updated");
                    update_line(self.blocks.iter_mut());
                    print!("blocks updated");
                    let pair = self.check_places();
                    update = pair.0;
                    solved = pair.1;
                }
                if !solved {
                    check_only_possible(self.rows.iter_mut());
                    check_only_possible(self.columns.iter_mut());
                    check_only_possible(self.blocks.iter_mut());
                    update = true;
                }
                cycles +=1;
                if cycles > 25 {
                    solved = true;
                }
            }
        }



        fn check_places(&mut self) -> (bool, bool) {
            let mut updated = false;
            let mut solved = true;

            for arc in self.places.iter_mut() {
                let mut p = arc.lock().unwrap();
                if p.value == None {
                    solved = false;
                    if p.count_possibles() == 1 {
                        p.set_from_possibles();
                        updated = true;
                    }
                }
            }
            return (updated, solved);
        }
    }

    fn check_only_possible(lines: IterMut<Line>) {

        for line in lines{

            let mut numbers : [usize; 9] = [0;9];

            for arc in line.places.iter(){
                let place  = arc.lock().unwrap();

                match place.value {
                    Some(i) => numbers[i-1] = 5,
                    None => {
                        for index in 0..8{
                            if place.possible[index] {
                                numbers[index] +=1;
                            }
                        }
                    }
                }
            }

            for index in 0..8 {

                if numbers[index] == 1 {
                    println!("Found one which is the only possible {}", index+1);

                    for arc in line.places.iter(){
                        let mut place = arc.lock().unwrap();
                        if place.possible[index] {
                            place.possible = [false;9];
                            place.possible[index] = true;
                            place.set_from_possibles();
                        }
                    }

                }

            }

        }
    }

    fn update_line(iter: IterMut<'_, Line>) {
        for l in iter {
            print!("Line -->>");
            l.pretty();

            let mut found = Vec::new();

            for p in l.places.iter() {
                if p.lock().unwrap().value != None {
                    found.push(p.lock().unwrap().value.unwrap());
                }
            }

            println!("Found items {:?}", found);

            for arc in l.places.iter_mut() {
                let mut p = arc.lock().unwrap();
                if p.value == None {
                    p.update(found.iter());
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::Borrow;
    use std::sync::Arc;
    use std::sync::Mutex;

    use crate::model::model::{Grid, Place};

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
        let arc = g.places.get(0).unwrap();
        arc.lock().unwrap().pretty();
    }

    #[test]
    fn test_with_easy() {
        print!("test with easy");
        let mut grid = Grid::new(&generate_4338937849());
        let result = solve_grid(&mut grid);

        let expected = "|1|5|2|9|7|6|8|4|3|\n\
                            |4|6|9|3|5|8|7|2|1|\n\
                            |3|7|8|4|1|2|6|9|5|\n\
                            |7|2|5|8|9|1|4|3|6|\n\
                            |9|3|1|7|6|4|2|5|8|\n\
                            |8|4|6|2|3|5|1|7|9|\n\
                            |2|1|4|5|8|3|9|6|7|\n\
                            |5|8|7|6|2|9|3|1|4|\n\
                            |6|9|3|1|4|7|5|8|2|\n";

        assert_eq!(result, expected);

    }

    #[test]
    fn test_with_medium() {
        print!("test with medium: https://www.websudoku.com/?level=2&set_id=6652955940");
        let mut grid = Grid::new(&generate_6652955940());
        let result = solve_grid(&mut grid);

        let expected =
                       "|8|6|1|7|9|5|3|4|2|\n\
                        |5|2|7|1|4|3|8|9|6|\n\
                        |4|9|3|6|2|8|1|7|5|\n\
                        |9|8|2|3|6|1|4|5|7|\n\
                        |7|1|5|4|8|9|6|2|3|\n\
                        |6|3|4|2|5|7|9|8|1|\n\
                        |3|5|9|8|7|6|2|1|4|\n\
                        |2|7|6|9|1|4|5|3|8|\n\
                        |1|4|8|5|3|2|7|6|9|\n";

        assert_eq!(result, expected);

    }

    #[test]
    fn test_with_hard() {
        print!("test with hard: https://www.websudoku.com/?level=3&set_id=8346528215");
        let mut grid = Grid::new(&generate_8346528215());
        let result = solve_grid(&mut grid);

        let expected =
               "|8|5|2|4|9|7|6|1|3|\n\
                |9|1|7|6|5|3|2|8|4|\n\
                |6|3|4|1|2|8|9|7|5|\n\
                |1|9|3|8|6|5|7|4|2|\n\
                |7|4|5|2|1|9|3|6|8|\n\
                |2|6|8|3|7|4|1|5|9|\n\
                |3|2|6|5|4|1|8|9|7|\n\
                |5|7|1|9|8|2|4|3|6|\n\
                |4|8|9|7|3|6|5|2|1|\n";

        assert_eq!(result, expected);

    }

    #[test]
    fn test_with_imposible() {
        print!("test with hard: https://www.websudoku.com/?level=4&set_id=3495689592");
        let mut grid = Grid::new(&generate_3495689592());
        let result = solve_grid(&mut grid);

        let expected =
            "|8|5|2|4|9|7|6|1|3|\n\
                |9|1|7|6|5|3|2|8|4|\n\
                |6|3|4|1|2|8|9|7|5|\n\
                |1|9|3|8|6|5|7|4|2|\n\
                |7|4|5|2|1|9|3|6|8|\n\
                |2|6|8|3|7|4|1|5|9|\n\
                |3|2|6|5|4|1|8|9|7|\n\
                |5|7|1|9|8|2|4|3|6|\n\
                |4|8|9|7|3|6|5|2|1|\n";

        assert_eq!(result, expected);

    }


    fn solve_grid(grid: &mut Grid) -> String {
        print!("Grid was generated");
        // let p = grid.places.get(0).unwrap();
        grid.pretty();

        for arc in grid.places.iter() {
            let p = arc.lock().unwrap();
            match p.value {
                None => assert_eq!(p.count_possibles(), 9),
                Some(_) => assert_eq!(p.count_possibles(), 1),
            }
        }

        grid.solve();

        let result = grid.pretty();

        for arc in grid.places.iter() {
            assert_ne!(arc.lock().unwrap().value, None);
        }
        result
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

    #[test]
    fn test_cell() {
        let p = Place::new(0);
        let c1 = Arc::new(Mutex::new(p));
        let c2 = Arc::clone(&c1);

        let mut v = Vec::new();
        for i in 1..5 {
            v.push(i);
        }

        for i in 6..10 {
            v.push(i);
        }

        println!("Vector is {:?}", v);


        c1.lock().unwrap().update(v.iter());
        c1.lock().unwrap().set_from_possibles();


        assert_eq!(c1.lock().unwrap().value, Some(5), "First ref not updated");
        assert_eq!(c2.lock().unwrap().value, Some(5), "Second ref not updated");
    }

    fn generate_empty_line() -> String {
        let zero_fn = || return 0;
        return generate_line(zero_fn);
    }

    fn generate_6652955940() -> String {
        let s= "\
        0,6,0,7,9,0,3,4,0,\
        0,0,0,1,0,0,8,9,0,\
		0,0,3,6,0,0,1,7,0,\
		0,0,0,0,0,1,0,0,0,\
		7,0,5,0,0,0,6,0,3,\
		0,0,0,2,0,0,0,0,0,\
		0,5,9,0,0,6,2,0,0,\
		0,7,6,0,0,4,0,0,0,\
		0,4,8,0,3,2,0,6,0".to_string();
        return s;

    }

    fn generate_4338937849() -> String {
        let s = "\
                     1,5,2,0,0,6,8,0,0,\
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

    fn generate_8346528215() -> String {
        let s = "\
                     8,0,0,0,0,0,0,0,0,\
                     9,0,7,0,0,3,0,0,4,\
                     0,3,4,0,2,0,0,7,5,\
                     0,0,0,8,6,0,0,4,0,\
                     0,0,0,2,0,9,0,0,0,\
                     0,6,0,0,7,4,0,0,0,\
                     3,2,0,0,4,0,8,9,0,\
                     5,0,0,9,0,0,4,0,6,\
                     0,0,0,0,0,0,0,0,1"
            .to_string();
        return s;
    }

    fn generate_3495689592() -> String {
        let s = "\
                     0,0,5,0,8,0,0,0,0,\
                     0,0,1,4,0,0,0,0,0,\
                     7,2,4,0,0,1,0,0,0,\
                     0,9,0,6,0,0,0,7,0,\
                     0,3,0,0,7,0,0,9,0,\
                     0,4,0,0,0,3,0,2,0,\
                     0,0,0,5,0,0,8,4,2,\
                     0,0,0,0,0,8,6,0,0,\
                     0,0,0,0,2,0,9,0,0"
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
