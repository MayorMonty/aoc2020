mod input;
mod day1;
mod day2;

extern crate regex;
#[macro_use] extern crate lazy_static;

fn main() {
    let answer = day2::run();

    println!("The day 2 solution is {:?}", answer);
}
