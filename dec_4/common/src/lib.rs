#[macro_use] extern crate lazy_static;
extern crate regex;
use regex::Regex;

pub mod day_four {
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

    fn is_present<T>(opt: Option<T>) -> bool {
        match opt {
            Some(_) => true,
            None => false
        }
    }

    fn get_single_with_pattern(pattern: &str, text: &str) -> Option<&str> {
        lazy_static! {
            static ref RE: Regex = match Regex::new(pattern) {
                Some(r) => r,
                None => panic!("Invalid pattern {}", pattern)
            };            
        }

        if !RE.is_match(text) {
            return None;
        }

        let captures = RE.captures_iter(captures);

        if captures.count() > 1 {
            return None
        }

        return captures.get(0);
    }

    impl Passport<'_> {
        fn from_text<'a>(text: &'a str) -> Passport {
            // TODO: do patterns
            return Passport {
                birth_year: get_single_with_pattern("byr", text),
                issue_year: get_single_with_pattern("iyr", text),
                expiration_year: get_single_with_pattern("eyr", text),
                height: get_single_with_pattern("hgt", text),
                hair_color: get_single_with_pattern("hcl", text),
                eye_color: get_single_with_pattern("ecl", text),
                passport_id: get_single_with_pattern("pid", text),
                country_id: get_single_with_pattern("cid", text)
            }
        }

        fn is_valid(&self) -> bool {
            is_present(self.birth_year)
            && is_present(self.issue_year)
            && is_present(self.expiration_year)
            && is_present(self.height)
            && is_present(self.hair_color)
            && is_present(self.passport_id)
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    #[test]
    fn it_parses_properly() {
        let input = read_test_file();
        
        assert_eq!(true, !input.is_empty());
    }

    fn read_test_file() -> String {
        match fs::read_to_string("./test") {
            Ok(s) => s,
            Err(_) => panic!("Could not read test file")
        }
    }
}
