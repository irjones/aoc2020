pub mod day_five {

    #[derive(Debug)]
    pub struct Seat {
        row: i32,
        column: i32
    }

    fn adjust_boundary(a: i32, b: i32) -> i32 {
        (a + b) / 2
    }

    impl Seat {
        pub fn id(&self) -> i32 {
            self.row * 8 + self.column
        }

        pub fn from(pass: &'_ str) -> Seat {
            let mut upper_row = 127;
            let mut lower_row = 0;
            let mut upper_column = 7;
            let mut lower_column = 0;

            for c in pass.chars() {
                match c {
                    'F' => upper_row = adjust_boundary(lower_row, upper_row),
                    'B' => lower_row = adjust_boundary(lower_row, upper_row),
                    'L' => upper_column = adjust_boundary(lower_column, upper_column),
                    'R' => lower_column = adjust_boundary(lower_column, upper_column),
                    _ => print!("{} not recognized!", c)
                }
            }

            dbg!(lower_row, lower_column);

            Seat {
                row: upper_row,
                column: upper_column
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::day_five::Seat;

    use std::fs;
    use std::collections::HashSet;

    #[test]
    fn it_creates_seat_correctly() {
        let input = "BFFFBBFRRR";
        let seat = Seat::from(input);
        assert_eq!(567, seat.id());
    }

    #[test]
    fn it_does_pt1_correctly() {
        let input = fs::read_to_string("./input").expect("Could not read file");
        let result = input.split("\n")
            .filter(|s| !s.is_empty())
            .map(|s| Seat::from(s))
            .map(|seat| seat.id())
            .max().expect("No value where value was expected");
        assert_eq!(989, result);
    }

    #[test]
    fn it_does_pt2_correctly() {
        let input = fs::read_to_string("./input").expect("Could not read file");
        let id_set = input.split('\n')
            .filter(|s| !s.is_empty())
            .map(|s| Seat::from(s))
            .map(|seat| seat.id())
            .collect::<HashSet<_>>();
        let min = id_set.iter().min().expect("No min where min expected");
        let max = id_set.iter().max().expect("No max where max expected");

        let mut answer: Option<i32> = None;
        for i in *min..*max {
            if !id_set.contains(&i) {
                answer = Some(i);
            }
        }

        assert_eq!(true, answer.is_some());
        assert_eq!(548, answer.unwrap());
    }
}
