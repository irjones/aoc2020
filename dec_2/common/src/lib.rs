pub mod day_two {
    // TODO: use the actual errors by validating the final strings before init Entry in ::new
    #[derive(Debug)]
    pub enum EntryConstructionError {
        PolicyParsingError,
        PasswordParsingError,
        InvalidEntryString,
    }

    pub type EntryResult<'a> = Result<Entry<'a>, EntryConstructionError>;

    #[derive(Debug)]
    pub struct Entry<'a> {
        required_letter: &'a str,
        range: (u16, u16),
        password: &'a str,
    }

    impl Entry<'_> {
        pub fn has_letter_enough(&self) -> bool {
            let letter_count = self.password.matches(self.required_letter).count() as u16;
            let is_valid = letter_count >= self.range.0 && letter_count <= self.range.1;
            print!(
                "\nChecking {:?} - counted {} of {} - isValid: {}",
                self, letter_count, self.required_letter, is_valid
            );
            is_valid
        }

        pub fn has_letter_in_correct_pos(&self) -> bool {
            let letters = self.password.split("").collect::<Vec<&str>>();
            let match_1 = match letters.get((self.range.0) as usize) {
                Some(s) => *s == self.required_letter,
                None => false,
            };
            let match_2 = match letters.get((self.range.1) as usize) {
                Some(s) => *s == self.required_letter,
                None => false,
            };

            (match_1 && !match_2) || (!match_1 && match_2)
        }

        pub fn new(raw: &'_ str) -> EntryResult<'_> {
            // E.G.: { 3-5 h: hhhhfhh }
            let parts = raw
                .split(':') // now is ["3-5 h", " hhhhfhh"]
                .collect::<Vec<&str>>();
            let policy_chunks = match parts.get(0) {
                Some(s) => s,
                None => "",
            }
            .split(' ') // gives us ["3-5", "h"]
            .collect::<Vec<_>>();
            let range_vec: Vec<u16> = match policy_chunks.get(0) {
                Some(s) => s,
                None => "",
            }
            .split('-')
            .collect::<Vec<_>>()
            .iter()
            .map(|s| match s.parse::<u16>() {
                Ok(i) => i,
                Err(_) => 0,
            })
            .collect();
            let password: &str = match parts.get(1) {
                Some(s) => s.trim(),
                None => "",
            };

            // TODO: error checking here

            return Result::Ok(Entry {
                required_letter: match policy_chunks.get(1) {
                    Some(chr) => &**chr,
                    None => &"",
                },
                range: (range_vec[0], range_vec[1]),
                password,
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::day_two::Entry;

    #[test]
    fn test_has_letter_enough_when_valid_returns_true() {
        let valid_entry = Entry::new("2-9 c: ccccccccc").unwrap();
        assert_eq!(true, valid_entry.has_letter_enough());
    }

    #[test]
    fn test_has_letter_enough_when_invalid_returns_false() {
        let invalid_entry = Entry::new("1-3 b: cdefg").unwrap();
        assert_eq!(false, invalid_entry.has_letter_enough());
    }

    #[test]
    fn test_has_letter_in_correct_pos_when_valid_returns_true() {
        let valid_entry = Entry::new("1-3 c: cdefg").unwrap();
        assert_eq!(true, valid_entry.has_letter_in_correct_pos());
    }

    #[test]
    fn test_has_letter_in_correct_pos_when_more_than_one_returns_false() {
        let valid_entry = Entry::new("1-3 c: cdcfg").unwrap();
        assert_eq!(false, valid_entry.has_letter_in_correct_pos());
    }

    #[test]
    fn test_has_letter_in_correct_pos_when_not_present_returns_false() {
        let valid_entry = Entry::new("1-3 h: cdefg").unwrap();
        assert_eq!(false, valid_entry.has_letter_in_correct_pos());
    }
}
