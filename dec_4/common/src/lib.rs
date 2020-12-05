    #[macro_use] extern crate lazy_static;
    extern crate regex;

pub mod day_four {
    use regex::Regex;

    pub fn parse_passports<'a>(input: &'a str) -> Vec<Passport<'a>> {
        lazy_static! {
            static ref RE: Regex = Regex::new("(.+\n)+").unwrap();
        }

        return RE.find_iter(&input)
            .map(|mat| mat.as_str())
            .map(|s| Passport::from_text(s)).collect();
    }

    #[derive(Debug)]
    pub struct Passport<'a> {
        birth_year: Option<&'a str>,
        issue_year: Option<&'a str>,
        expiration_year: Option<&'a str>,
        height: Option<&'a str>,
        hair_color: Option<&'a str>,
        passport_id: Option<&'a str>,
        eye_color: Option<&'a str>,
        country_id: Option<&'a str>
    }

    fn get_single_with_pattern<'a>(pattern: &str, text: &'a str) -> Option<&'a str> {

        let re: Regex = match Regex::new(pattern) {
            Ok(r) => r,
            Err(_) => panic!("Invalid pattern {}", pattern)
        };            

        if !re.is_match(text) {
            return None;
        }

        let captures: Vec<&str> = re.find_iter(text)
            .map(|cap| cap.as_str())
            .map(|s| match s.split(':').collect::<Vec<&str>>().get(1) {
                Some(s) => *s,
                None => ""
            })
            .collect();

        // if more than 1, oops
        if captures.len() > 1 {
            return None
        }

        //deref by one level so I don't have Option<&&str>
        return match captures.get(0) {
            Some(s) => Some(*s), 
            None => None
        }
    }

    impl Passport<'_> {
        fn from_text<'a>(text: &'a str) -> Passport {
            return Passport {
                birth_year: get_single_with_pattern("(?:byr:)[^\\s]+", text),
                issue_year: get_single_with_pattern("(?:iyr:)[^\\s]+", text),
                expiration_year: get_single_with_pattern("(?:eyr:)[^\\s]+", text),
                height: get_single_with_pattern("(?:hgt:)[^\\s]+", text),
                hair_color: get_single_with_pattern("(?:hcl:)[^\\s]+", text),
                eye_color: get_single_with_pattern("(?:ecl:)[^\\s]+", text),
                passport_id: get_single_with_pattern("(?:pid:)[^\\s]+", text),
                country_id: get_single_with_pattern("(?:cid:)[^\\s]+", text)
            }
        }

        pub fn is_valid(&self) -> bool {
            self.birth_year.is_some()
            && self.issue_year.is_some()
            && self.expiration_year.is_some()
            && self.height.is_some()
            && self.hair_color.is_some()
            && self.eye_color.is_some()
            && self.passport_id.is_some()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::day_four::parse_passports;
    use std::fs;

    #[test]
    fn it_parses_properly() {
        let input = read_test_file();
        
        let passports = parse_passports(&input);
        dbg!(&passports);
        assert_eq!(4, passports.len());
    }

    #[test]
    fn it_gives_true_for_valid_passport() {
        let input = read_test_file();

        let passports = parse_passports(&input);
        let valid_passport = match passports.get(2) {
            Some(p) => p,
            None => panic!("Passports not parsed correctly")
        };

        assert_eq!(true, valid_passport.is_valid())
    }

    #[test]
    fn it_gives_false_for_invalid_passport() {
        let input = read_test_file();

        let passports = parse_passports(&input);
        let invalid_passport = match passports.get(1) {
            Some(p) => p,
            None => panic!("Passports not parsed correctly")
        };

        assert_eq!(false, invalid_passport.is_valid())
    }

    #[test]
    fn it_gets_test_input_right() {
        let input = read_test_file();
        let passports = parse_passports(&input);
        let result = passports.iter()
            .filter(|s| s.is_valid())
            .count();
        assert_eq!(2, result);
    }

    fn read_test_file() -> String {
        match fs::read_to_string("./test") {
            Ok(s) => s,
            Err(_) => panic!("Could not read test file")
        }
    }
}
