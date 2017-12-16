extern crate time;

use time::PreciseTime;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;

fn main() {
    let start = PreciseTime::now();
    day01::solve();
    day02::solve();
    day03::solve();
    day04::solve();
    day05::solve();
    day06::solve();
    day07::solve();
    day08::solve();
    day09::solve();
    day10::solve();
    day11::solve();
    day12::solve();
    day13::solve();
    day14::solve();
    day15::solve();
    day16::solve();
    let end = PreciseTime::now();
    println!("\nTime: {} ms", start.to(end).num_milliseconds());
}