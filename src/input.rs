use std::{
    fs,
    io::BufRead,
    io::{self, BufReader},
    path::Path,
    str::{FromStr},
};

pub fn from_file<'a, S: FromStr>(filename: &'static str, delimiter: &'a str) -> io::Result<Vec<S>>
where
    <S as std::str::FromStr>::Err: std::fmt::Debug,
{   
    // Read to string, and then split based on delimiter
    let contents = fs::read_to_string(filename)?;

    // Split a the delimiter, and then parse individual items according to the
    // FromStr trait
    Ok(contents.split(delimiter)
        .into_iter()
        .map(|item| item.parse::<S>().unwrap())
        .collect())
}
