#![allow(dead_code)]
mod day1;
mod day2;
mod day3;
mod day4;
mod input;

fn main() {
    let a = day4::run_a();
    let b = day4::run_b();

    println!("The day 4A solution is {:?}", a);
    println!("The day 4B solution is {:?}", b);
}
