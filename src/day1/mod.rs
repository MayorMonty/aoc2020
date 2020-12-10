use crate::input;
use std::io;

pub fn run() -> io::Result<usize> {   
    let items = input::from_file::<usize>(&"src/day1/input.txt", "\n")?;

    for a in &items {
        for b in &items {
            for c in &items {
                if a + b + c == 2020 {
                    return Ok(a * b * c);
                }
            }
        }
    }

    Err(io::Error::from(io::ErrorKind::InvalidInput))
}
