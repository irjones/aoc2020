use std::env;
use std::fs;

enum EntryConstructionError {
    PolicyParsingError,
    PasswordParsingError,
    InvalidEntryString
}

type EntryResult<'a> = Result<Entry<'a>, EntryConstructionError>;

#[derive(Debug)]
struct Entry<'a> {
    required_letter: &'a str,
    range: (u16, u16),
    password: &'a str,
}

impl Entry<'_> {
    fn is_valid(&self) -> bool {
        let letter_count = self.password.matches(self.required_letter).count() as u16;
        let is_valid = letter_count >= self.range.0 && letter_count <= self.range.1;
        print!("\nChecking {:?} - counted {} of {} - isValid: {}", self, letter_count, self.required_letter, is_valid);
        return is_valid;
    }

    fn new<'a>(raw: &'a str) -> EntryResult<'a> {
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

fn main() {
    let args: Vec<String> = env::args().collect();
    let filepath = args.get(1).expect("No filepath given!");

    let content = match fs::read_to_string(filepath) {
            Ok(contents) => contents,
            Err(err) => panic!("Unable to read file: {}", err)
        };

    let valid_entries = content
        .split("\n")
        .collect::<Vec<&str>>()
        .iter()
        .filter(|&s| s.len() > 0)
        .map(|&s| Entry::new(s))
        .filter(|er| match er {
            Ok(e) => e.is_valid(),
            Err(_) => false
        })
        .collect::<Vec<_>>();
    
    print!("\n---\nValid Entries: {}\n", valid_entries.len());
}
