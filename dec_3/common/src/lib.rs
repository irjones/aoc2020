pub mod day_three {
    #[derive(Debug)]
    pub struct Slope {
        pub down: usize,
        pub right: usize,
    }

    impl Slope {
        pub fn of(x: usize, y: usize) -> Self {
            Slope { down: y, right: x }
        }
    }

    #[derive(Debug)]
    struct Coords {
        x: usize,
        y: usize,
    }

    #[derive(Debug)]
    pub struct GeoMap {
        pub map_grid: Vec<Vec<char>>,
        self_width: usize,
        self_height: usize,
    }

    impl GeoMap {
        pub fn from_text(text: &'_ str) -> GeoMap {
            let map_grid: Vec<Vec<char>> = text
                .split('\n')
                .collect::<Vec<_>>()
                .iter()
                .filter(|s| !s.is_empty())
                .map(|s| s.chars().collect())
                .collect::<Vec<_>>();

            let self_width = match map_grid.get(0) {
                Some(vec_char) => vec_char.len(),
                None => 0,
            };

            let self_height = map_grid.len();

            GeoMap {
                map_grid,
                self_width,
                self_height,
            }
        }

        pub fn traverse_with_slope(&self, slope: Slope) -> i32 {
            let mut x: usize = 0;
            let mut y: usize = 0;

            let mut coordinates: Vec<Coords> = Vec::new();
            while y < self.self_height {
                let coords = self.get_next(Coords { x, y }, &slope);
                x = coords.x;
                y = coords.y;
                coordinates.push(coords)
            }

            let path = coordinates
                .iter()
                .map(|coords| match self.map_grid.get(coords.y) {
                    Some(v) => match v.get(coords.x) {
                        Some(c) => c,
                        None => &'_',
                    },
                    None => &'_',
                })
                .collect::<Vec<_>>();

            let trees = path.iter().filter(|c| ***c == '#').count() as i32;

            trees
        }

        fn get_next(&self, current_pos: Coords, slope: &Slope) -> Coords {
            Coords {
                x: (current_pos.x + slope.right) % self.self_width,
                y: current_pos.y + slope.down,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::day_three::{GeoMap, Slope};
    use std::fs;

    fn get_test_input() -> String {
        match fs::read_to_string("./test") {
            Ok(input) => input,
            Err(_) => String::from(""),
        }
    }

    #[test]
    fn it_can_init_properly() {
        let expected_len = 11;
        let input = get_test_input();
        let geo_map = GeoMap::from_text(&input);
        assert_eq!(expected_len, geo_map.map_grid.len());
        assert_eq!(expected_len, geo_map.map_grid[0].len());
    }

    #[test]
    fn it_can_traverse_properly() {
        let expected_trees = 7;
        let slope = Slope::of(3, 1);
        let input = get_test_input();
        let geo_map = GeoMap::from_text(&input);
        assert_eq!(expected_trees, geo_map.traverse_with_slope(slope))
    }

    #[test]
    fn it_gets_pt_1_right() {
        let expected_trees = 284;
        let slope = Slope::of(3, 1);
        let input = match fs::read_to_string("../input") {
            Ok(content) => content,
            Err(e) => panic!("Did not read file: {}", e),
        };

        let geo_map = GeoMap::from_text(&input);
        assert_eq!(expected_trees, geo_map.traverse_with_slope(slope));
    }
}
