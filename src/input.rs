use std::{
    fs::File,
    io::BufRead,
    io::{self, BufReader},
    path::Path,
    str::FromStr,
};

pub fn from_file<T: FromStr>(filename: &'static str, delimiter: u8) -> io::Result<Vec<T>>
where
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let path = Path::new(filename);
    let file = File::open(&path)?;

    // Read the file line by line
    let lines = BufReader::new(file).split(delimiter).map(|l| String::from_utf8(l.unwrap()));

    // Create a vector to hold the numbers, with max capacity of the lower bound
    let mut items: Vec<T> = Vec::with_capacity(lines.size_hint().0);
    for line in lines {
        if let Ok(num) = line {
            let num = num.parse::<T>().unwrap();
            items.push(num);
        };
    }

    Ok(items)
}
