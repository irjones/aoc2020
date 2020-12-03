pub mod day_three {
    pub struct Slope {
        down: usize,
        right: usize
    }

    #[derive(Debug)]
    pub struct GeoMap {
        pub map_grid: Vec<Vec<char>>
    }

    impl GeoMap {
        pub fn from_text(text: &'_ str) -> GeoMap {
            let map_grid: Vec<Vec<char>> = text.split('\n')
                .collect::<Vec<_>>()
                .iter()
                .filter(|s| !s.is_empty())
                .map(|s| s.chars().collect())
                .collect::<Vec<_>>();

            GeoMap {
                map_grid
            }
        }

        pub fn traverse_with_slope(&self, slope: Slope) -> i32 {
            1
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use crate::day_three::GeoMap;

    fn get_test_input() -> String {
        match fs::read_to_string("./test") {
            Ok(input) => input,
            Err(_) => String::from("")
        }
    }

    #[test]
    fn it_can_init_properly() {
        let expected_len = 11;
        let input = get_test_input();
        let geo_map = GeoMap::from_text(&input);
        dbg!(&geo_map);
        assert_eq!(expected_len, geo_map.map_grid.len());
        assert_eq!(expected_len, geo_map.map_grid[0].len());
    }
}
