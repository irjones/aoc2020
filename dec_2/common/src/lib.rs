pub mod day_two {
    // TODO: use the actual errors by validating the final strings before init Entry in ::new
    pub enum EntryConstructionError {
        PolicyParsingError,
        PasswordParsingError,
        InvalidEntryString
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
            print!("\nChecking {:?} - counted {} of {} - isValid: {}", self, letter_count, self.required_letter, is_valid);
            return is_valid;
        }

        pub fn has_letter_in_correct_pos(&self) -> bool {
            let letters = self.password.split("").collect::<Vec<&str>>();
            let match_1 = match letters.get((self.range.0) as usize) {
                Some(s) => *s == self.required_letter,
                None => false
            };
            let match_2 = match letters.get((self.range.1) as usize) {
                Some(s) => *s == self.required_letter,
                None => false
            };

            return (match_1 && !match_2) || (!match_1 && match_2);
        }

        pub fn new<'a>(raw: &'a str) -> EntryResult<'a> {
            // E.G.: { 3-5 h: hhhhfhh }
            let parts = 
                raw.split(":") // now is ["3-5 h", " hhhhfhh"]
                    .collect::<Vec<&str>>();
            let policy_chunks = match parts.get(0) {
                    Some(s) => s,
                    None => ""
                }
                .split(" ") // gives us ["3-5", "h"]
                .collect::<Vec<_>>();
            let range_vec: Vec<u16> = match policy_chunks.get(0) {
                    Some(s) => s,
                    None => ""
                }
                .split("-")
                .collect::<Vec<_>>()
                .iter()
                .map(|s| match s.parse::<u16>() {
                    Ok(i) => i,
                    Err(_) => 0
                })
                .collect();
            let password: &str = match parts.get(1) {
                Some(s) => s,
                None => ""
            };

            // TODO: error checking here

            return Result::Ok(
                Entry {
                    required_letter: match policy_chunks.get(1) {
                        Some(chr) => &**chr,
                        None => &""
                    },
                    range: (
                        range_vec[0],
                        range_vec[1]
                    ),
                    password: parts[1].trim(),
                }
            )
        }
    }    
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
