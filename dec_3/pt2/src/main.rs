extern crate common;
use std::env;
use std::fs;
use common::day_three::{GeoMap, Slope};

fn main() {
    let args: Vec<String> = env::args().collect();

    let filepath = args.get(1).expect("Could not read file");

    let content = match fs::read_to_string(filepath) {
        Ok(s) => s,
        Err(_) => String::from("")
    };
    let geo_map = GeoMap::from_text(&content);

    let slopes = vec!(
        Slope::of(3, 1),
        Slope::of(1, 1),
        Slope::of(5, 1),
        Slope::of(7, 1),
        Slope::of(1, 2)
    );

    let mut result_total: i64 = 1;

    for slope in slopes {
        print!("Slope {:?}", slope);
        let result = geo_map.traverse_with_slope(slope);
        print!("\n\t >>> Trees: {}", result);
        result_total = result_total * (result as i64);
    }

    print!("Result total: {}", result_total);
}
