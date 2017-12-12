use std::fs::File;
use std::io::prelude::*;
use std::error::Error;
use std::usize;

fn load_input() -> Result<String, Box<Error>> {
    let mut file = File::open("src/day02/input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    return Ok(contents);
}

fn part1(input: &String) -> usize {
    let mut sum = 0;
    for line in input.split("\n") {
        let mut min = usize::MAX;
        let mut max = 0;

        for number in line.split("\t") {
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

    return sum;
}

fn part2(input: &String) -> usize {
    let mut sum = 0;
    for line in input.split("\n") {
        let mut numbers = vec![];

        for number in line.split("\t") {
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

    return sum;
}

pub fn solve() {
    let text = load_input().expect("Error while reading file");
    let solution1 = part1(&text);
    let solution2 = part2(&text);
    println!("{} {}", solution1, solution2);
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        assert_eq!(18, super::part1(&"5\t1\t9\t5\n7\t5\t3\n2\t4\t6\t8".to_string()));
    }

    #[test]
    fn part2() {
        assert_eq!(9, super::part2(&"5\t9\t2\t8\n9\t4\t7\t3\n3\t8\t6\t5".to_string()));  
    }
}