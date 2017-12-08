extern crate time;

use time::PreciseTime;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;

fn main() {
    let start = PreciseTime::now();
    day1::solve();
    day2::solve();
    day3::solve();
    day4::solve();
    day5::solve();
    day6::solve();
    day7::solve();
    day8::solve();
    let end = PreciseTime::now();
    println!("\nTime: {} ms", start.to(end).num_milliseconds());
}