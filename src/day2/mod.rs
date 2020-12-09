use crate::input;
use std::{io, str::FromStr};
use regex::{Regex, Captures};

#[derive(Debug)]
pub struct Password {

    policy_min: usize,
    policy_max: usize,
    policy_char: char,

    password: String,
}

impl FromStr for Password {
    type Err = io::Error;

    fn from_str<'a>(encoding: &'a str) -> Result<Self, Self::Err> {

        // Split at delimiters to extract format
        // NOTE: this code is pretty brittle, and really only works for ASCII
        // values, but should be acceptable for this challenge
        let (min, encoding) = encoding.split_at(encoding.find('-').unwrap());
        let encoding = &encoding[1..];
        let (max, encoding) = encoding.split_at(encoding.find(' ').unwrap());
        let encoding = &encoding[1..];
        let (ch, encoding) = encoding.split_at(1);
        let password = (&encoding[2..]).to_string();
        
        Ok(Password {
            policy_min: min.parse().unwrap(),
            policy_max: max.parse().unwrap(),
            policy_char: ch.chars().next().unwrap(),
            password
        })

    }

}


// Validates a password to see if it's compliant with spec A (a min and max
// occurence of a specific char )
pub fn validateA(password: &Password) -> bool {
    let occurances = password.password.matches(password.policy_char).count();

    password.policy_min <= occurances && occurances <= password.policy_max
}

// Validates a password against spec B (a char must exist in 1 of 2 places, but
// not both)
pub fn validateB(password: &Password) -> bool {
    
    let in_first = password.password.chars().nth(password.policy_min - 1).unwrap_or('\0') == password.policy_char;
    let in_second =  password.password.chars().nth(password.policy_max - 1).unwrap_or('\0') == password.policy_char;
    
    return in_first ^ in_second;

}

pub fn run() -> io::Result<usize> {   
    let items = input::from_file::<Password>(&"src/day2/input.txt")?;

    let mut countA = 0;
    let mut countB = 0;

    for pw in items {
        if validateA(&pw) { countA += 1 };
        if validateB(&pw) { countB += 1 };
    }

    Ok(countB)
}
