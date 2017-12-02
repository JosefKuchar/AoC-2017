use std::fs::File;
use std::io::prelude::*;
use std::error::Error;
use std::io::BufReader;
use std::usize;

pub fn solve() {
    let f = File::open("src/day2/input.txt").unwrap();
    let file = BufReader::new(&f);
    let mut sum = 0;
    for line in file.lines() {
        let mut min = usize::MAX;
        let mut max = 0;

        for number in line.unwrap().split("\t") {
            let value = number.parse::<usize>().unwrap();
            if value < min {
                min = value;
            }

            if value > max {
                max = value;
            }
        }

        sum += max - min;
    }

    println!("{}", sum);
    let f = File::open("src/day2/input.txt").unwrap();
    let file = BufReader::new(&f);
    sum = 0;
    for line in file.lines() {
        let mut numbers = vec![];

        for number in line.unwrap().split("\t") {
            let value = number.parse::<usize>().unwrap();
            numbers.push(value);

            for pnumber in &numbers {
                if *pnumber > value {
                    if *pnumber % value == 0 {
                        sum += *pnumber / value;
                    }
                } else if *pnumber < value {
                    if value % *pnumber == 0 {
                        sum += value / *pnumber;
                    }
                }
            }
        }
    }

    println!("{}", sum);
}