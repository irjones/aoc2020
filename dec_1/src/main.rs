use std::env;
use std::fs;
use std::collections::HashSet;

fn main() {
    // get input file
    let filename: Vec<String> = env::args().collect();
    
    // read file into searchable format
    let inputs: Vec<i32> = match fs::read_to_string(filename.get(1).expect("No filename given!")) {
        Ok(s) => s,
        Err(_) => String::from("0")
    }
        .split("\n")
        .collect::<Vec<&str>>()
        .iter().map(|&s| match s.parse::<i32>() {
            Ok(i) => i,
            Err(_) => 0 // ignore a bad parse, was probably a new line
        })
        .filter(|&i| i != 0)
        .collect();

    let distinct_nums: HashSet<i32> = inputs.iter().cloned().collect();
    let mut candidates: (i32, i32, i32) = (0, 0, 0);

    // brute force to find which two nums sum to 2020...
    let mut done = false;
    // this is not efficient nor pretty
    for i in distinct_nums.iter() {
        if !done {
            for j in distinct_nums.iter() {
                if !done {
                    for k in distinct_nums.iter() {
                        // most of these checks are to see if we're comparing a number to itself
                        if i != j
                            && i != k 
                            && j != k 
                            && i + j + k == 2020 {
                            // once we've found it, we're good to jump out
                            candidates = (*i, *j, *k);
                            done = true;
                            break;
                        }                    
                    }
                }
            }            
        }

    }

    print!("Candidates: {:?}\n", candidates);
    print!(">>>\tT: {:?} = {}\n", candidates, candidates.0 * candidates.1 * candidates.2);
}
