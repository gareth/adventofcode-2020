#[macro_use]
extern crate lazy_static;
extern crate regex;

mod day1;
mod day2;

fn main() {
    println!("Day 1");
    day1::run("input/day1.txt");
    println!("Day 2");
    day2::run("input/day2.txt");
}
