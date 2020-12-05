#[macro_use]
extern crate lazy_static;
extern crate regex;

pub mod day_four {
    use regex::Regex;

    pub fn parse_passports<'a>(input: &'a str) -> Vec<Passport<'a>> {
        lazy_static! {
            static ref RE: Regex = Regex::new("(.+\n)+").unwrap();
        }

        return RE
            .find_iter(&input)
            .map(|mat| mat.as_str())
            .map(|s| Passport::from_text(s))
            .collect();
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
        country_id: Option<&'a str>,
    }

    fn validate_field(field: Option<&str>, validator_f: &dyn Fn(&str) -> bool) -> bool {
        let result = match field {
            Some(s) => validator_f(s),
            None => false,
        };
        if !result {
            print!("\n{:?} was not valid\n", field)
        }
        result
    }

    fn between_values(check_val: &str, a: i32, b: i32) -> bool {
        match check_val.parse::<i32>() {
            Ok(i) => i >= a && i <= b,
            Err(_) => false,
        }
    }

    fn get_single_match_by_pattern<'a>(pattern: &str, text: &'a str) -> Option<&'a str> {
        let re: Regex = match Regex::new(pattern) {
            Ok(r) => r,
            Err(_) => panic!("Invalid pattern {}", pattern),
        };

        if !re.is_match(text) {
            return None;
        }

        let captures: Vec<&str> = re.find_iter(text).map(|cap| cap.as_str()).collect();

        // if more than 1, oops
        if captures.len() > 1 {
            return None;
        }

        //deref by one level so I don't have Option<&&str>
        return match captures.get(0) {
            Some(s) => Some(*s),
            None => None,
        };
    }

    fn get_field_with_label<'a>(pattern: &str, text: &'a str) -> Option<&'a str> {
        let re: Regex = match Regex::new(pattern) {
            Ok(r) => r,
            Err(_) => panic!("Invalid pattern {}", pattern),
        };

        if !re.is_match(text) {
            return None;
        }

        let captures: Vec<&str> = re
            .find_iter(text)
            .map(|cap| cap.as_str())
            .map(|s| match s.split(':').collect::<Vec<&str>>().get(1) {
                Some(s) => *s,
                None => "",
            })
            .collect();

        // if more than 1, oops
        if captures.len() > 1 {
            return None;
        }

        //deref by one level so I don't have Option<&&str>
        return match captures.get(0) {
            Some(s) => Some(*s),
            None => None,
        };
    }

    impl Passport<'_> {
        fn from_text(text: &'_ str) -> Passport {
            return Passport {
                birth_year: get_field_with_label("(?:byr:)[^\\s]+", text),
                issue_year: get_field_with_label("(?:iyr:)[^\\s]+", text),
                expiration_year: get_field_with_label("(?:eyr:)[^\\s]+", text),
                height: get_field_with_label("(?:hgt:)[^\\s]+", text),
                hair_color: get_field_with_label("(?:hcl:)[^\\s]+", text),
                eye_color: get_field_with_label("(?:ecl:)[^\\s]+", text),
                passport_id: get_field_with_label("(?:pid:)[^\\s]+", text),
                country_id: get_field_with_label("(?:cid:)[^\\s]+", text),
            };
        }

        pub fn has_required_fields(&self) -> bool {
            self.birth_year.is_some()
                && self.issue_year.is_some()
                && self.expiration_year.is_some()
                && self.height.is_some()
                && self.hair_color.is_some()
                && self.eye_color.is_some()
                && self.passport_id.is_some()
        }

        pub fn fields_are_valid(&self) -> bool {
            validate_field(self.birth_year, &|byr: &str| {
                between_values(byr, 1920, 2002)
            }) && validate_field(self.issue_year, &|iyr: &str| {
                between_values(iyr, 2010, 2020)
            }) && validate_field(self.expiration_year, &|eyr| between_values(eyr, 2020, 2030))
                && validate_field(self.height, &|hgt| {
                    let val = match get_single_match_by_pattern("\\d+", hgt) {
                        Some(s) => s,
                        None => "",
                    };
                    if hgt.contains("cm") {
                        between_values(val, 150, 193)
                    } else if hgt.contains("in") {
                        between_values(val, 59, 76)
                    } else {
                        false
                    }
                })
                && validate_field(self.hair_color, &|hcl| {
                    get_single_match_by_pattern("#[a-f\\d]{6}", hcl).is_some()
                })
                && validate_field(self.eye_color, &|ecl| {
                    get_single_match_by_pattern("(amb|blu|brn|gry|grn|hzl|oth)", ecl).is_some()
                })
                && validate_field(self.passport_id, &|pid| {
                    get_single_match_by_pattern("\\d{9}", pid).is_some()
                })
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
            None => panic!("Passports not parsed correctly"),
        };

        assert_eq!(true, valid_passport.has_required_fields())
    }

    #[test]
    fn it_gives_false_for_invalid_passport() {
        let input = read_test_file();

        let passports = parse_passports(&input);
        let invalid_passport = match passports.get(1) {
            Some(p) => p,
            None => panic!("Passports not parsed correctly"),
        };

        assert_eq!(false, invalid_passport.has_required_fields())
    }

    #[test]
    fn it_gets_test_input_right() {
        let input = read_test_file();
        let passports = parse_passports(&input);
        let result = passports.iter().filter(|s| s.has_required_fields()).count();
        assert_eq!(2, result);
    }

    #[test]
    fn it_correctly_marks_all_invalid_field_passports() {
        let input = read_custom_file("./invalid_passports");
        let passports = parse_passports(&input);
        let result = passports.iter().filter(|p| !p.fields_are_valid()).count();
        assert_eq!(4, result);
    }

    #[test]
    fn it_correctly_marks_all_valid_field_passports() {
        let input = read_custom_file("./valid_passports");
        let passports = parse_passports(&input);
        let result = passports.iter().filter(|p| p.fields_are_valid()).count();
        assert_eq!(4, result);
    }

    fn read_test_file() -> String {
        read_custom_file("./test")
    }

    fn read_custom_file(file: &str) -> String {
        fs::read_to_string(file).expect("Could not read file.")
    }
}
