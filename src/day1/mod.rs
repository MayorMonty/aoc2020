use std::io::BufRead;
use std::path::Path;
use std::{fs::File, io::BufReader};

pub fn run() -> Result<usize, ()> {
    let path = Path::new("src/day1/input.txt");
    let display = path.display();

    let file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    // Read the file line by line
    let lines = BufReader::new(file).lines();

    // Create a vector to hold the numbers, with max capacity of the lower bound
    let mut items: Vec<usize> = Vec::with_capacity(lines.size_hint().0);
    for line in lines {
        if let Ok(num) = line {
            let num = num.parse::<usize>().expect("Could not parse line as num");
            items.push(num);
        };
    }

    for a in &items {
        for b in &items {
            for c in &items {
                if a + b + c == 2020 {
                    return Ok(a * b * c);
                }
            }
        }
    }

    Err(())
}
