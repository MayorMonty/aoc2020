use crate::input;
use std::{io};

#[derive(Clone)]
pub struct TobogganArea {

    width: usize,
    height: usize,

    area: Vec<Vec<char>>,
    pointer: (usize, usize)
}

impl TobogganArea {

    fn from(area: Vec<String>) -> Self {
        TobogganArea {
            width: area[0].len(),
            height: area.len(),
            area: area.iter().map(|v| v.chars().collect()).collect(),
            pointer: (0, 0)
        }
    }

    fn advance(&mut self, x: usize, y: usize) {
        self.pointer = ((self.pointer.0 + x) % self.width, self.pointer.1 + y);
    }

    fn is_complete(&self) -> bool {
        self.pointer.1 >= self.height
    }

    fn char_at(&self) -> char {
        self.area[self.pointer.1][self.pointer.0]
    }

} 

pub fn num_trees(x: usize, y: usize, trip: &mut TobogganArea) -> usize {

    let mut num = 0;

    while !trip.is_complete() {
        if trip.char_at() == '#' { num += 1 };
        trip.advance(x, y)
    };

    num

}

pub fn run() -> io::Result<(usize, usize)> {   
    let items = input::from_file::<String>(&"src/day3/input.txt")?;
    let mut trip = TobogganArea::from(items);

    // Part A
    let part_a = num_trees(3, 1, &mut trip.clone());

    // PART B, for multiple slopes
    let trees: Vec<usize> = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .into_iter().map(|(x, y)| num_trees(x, y, &mut trip.clone())).collect();

    let mut part_b = 1;

    for num in trees {
        part_b *= num;
    }




    Ok((part_a, part_b))
}
