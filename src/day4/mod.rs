use crate::input;
use std::{collections::HashMap, convert::TryFrom, io, str::FromStr};

struct Passport(HashMap<String, String>);

impl FromStr for Passport {
    type Err = io::Error;

    fn from_str<'a>(encoding: &'a str) -> Result<Self, Self::Err> {
        let pairs = encoding.split_whitespace();

        let mut fields = HashMap::new();

        for pair in pairs {
            if let Some(delimit) = pair.find(":") {
                let (key, value) = pair.split_at(delimit);
                fields.insert(key.to_string(), value[1..].to_string());
            }
        }

        Ok(Passport(fields))
    }
}

static REQUIRED_FIELDS: &'static [&'static str] =
    &["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

impl Passport {
    /// Tests to ensure a field is present and valid on the passport. If the
    /// field is not valid or not present, returns false
    pub fn validate_field(&self, field: String) -> bool {
        if let Some(value) = self.0.get(&field) {
            match field.as_str() {
                "byr" => {
                    let value: usize = value.parse().unwrap_or(0);
                    1920 < value && value < 2002
                }
                "iyr" => {
                    let value: usize = value.parse().unwrap_or(0);
                    2010 < value && value < 2020
                }
                "eyr" => {
                    let value: usize = value.parse().unwrap_or(0);
                    2020 < value && value < 2030
                }
                "hgt" => {}
                "hcl" => true,
                "ecl" => {
                    ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&value.as_str())
                }
                "pid" => {
                    let is_number = (*value).parse::<usize>().is_ok();

                    is_number && value.len() == 9
                }
                _ => true,
            }
        } else {
            false
        }
    }
}

pub fn run_a() -> io::Result<usize> {
    let passports = input::from_file::<Passport>(&"src/day4/input.txt", "\n\n")?;

    let mut valid = 0;

    for passport in passports {
        let mut legal = true;

        for field in REQUIRED_FIELDS {
            if !passport.0.contains_key(*field) {
                legal = false;
            }
        }

        if legal {
            valid += 1;
        }
    }

    Ok(valid)
}

pub fn run_b() -> io::Result<usize> {
    let passports = input::from_file::<Passport>(&"src/day4/input.txt", "\n\n")?;

    let mut valid = 0;

    for passport in passports {
        let mut legal = true;

        for field in REQUIRED_FIELDS {
            if !passport.validate_field(field.to_string()) {
                legal = false;
            }
        }

        if legal {
            valid += 1;
        }
    }

    Ok(valid)
}
