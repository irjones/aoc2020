pub mod day_six {
    use std::collections::HashSet;

    fn group_to_sets(group: &str) -> HashSet<char> {
        let strings_by_person = group.split("\n").collect::<Vec<_>>();
        let chars_by_person = strings_by_person.iter().map(|s| s.chars()).collect::<Vec<_>>();

        let mut set = HashSet::new();

        for char_col in chars_by_person {
            for c in char_col {
                set.insert(c);
            }
        }

        dbg!(set)
    }

    fn reduce_to_intersection(sets: Vec<HashSet<char>>) -> HashSet<char> {
        let result = HashSet::new();
        for set in sets {
            // get intersection of all sets
            // TODO
        }
        result
    }

    pub fn pt1(text: &str) -> usize {
        text.split("\n\n")
            .map(group_to_sets)
            .map(|s| s.len())
            .fold(0, |acc, next| acc + next)
    }

    pub fn pt2(text: &str) -> usize {
        0 // TODO
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use crate::day_six::pt1;

    #[test]
    fn it_works_for_pt_1_test_input() {
        let test_input = fs::read_to_string("./test").expect("Could not read file");

        let result = pt1(&test_input);

        assert_eq!(11, result)
    }

    #[test]
    fn it_works_for_pt_1_real_input() {
        let test_input = fs::read_to_string("./input").expect("Could not read file");

        let result = pt1(&test_input);

        assert_eq!(6768, result)
    }

    #[test]
    fn it_works_for_pt_2_test_input() {

    }
}
