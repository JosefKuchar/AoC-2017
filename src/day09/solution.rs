use std::fs::File;
use std::io::prelude::*;
use std::error::Error;

fn load_input() -> Result<String, Box<Error>> {
    let mut file = File::open("src/day9/input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    return Ok(contents);
}


pub fn solve() {
    let contents = load_input().unwrap();
    let solution1 = part1(&contents);
    let solution2 = part2(&contents);
    println!("{} {}", solution1, solution2);
}

fn part1(input: &str) -> usize {
    let mut depth = 0;
    let mut sum = 0;
    let mut garbage = false;
    let mut cancel = false;
    for character in input.chars() {
        match character {
            '{' => {
                if !garbage {
                    depth += 1;
                    sum += depth;
                }
            },
            '}' => {
                if !garbage {
                    depth -= 1;
                }
            },
            '<' => {
                garbage = true
            },
            '>' => {
                if !cancel {
                    garbage = false
                }
            },
            '!' => {
                if !cancel {
                    cancel = true;
                    continue;
                }
            }
            _ => {}
        }
        cancel = false;
    }
    return sum;
}

fn part2(input: &str) -> usize {
    let mut garbage = false;
    let mut characters = 0;
    let mut cancel = false;
    for character in input.chars() {
        match character {
            '<' => {
                if !cancel && garbage {
                    characters += 1
                } else {
                    garbage = true;
                }
            },
            '>' => {
                if !cancel {
                    garbage = false;
                }
            },
            '!' => {
                if !cancel {
                    cancel = true;
                    continue;
                }
            }
            _ => {
                if !cancel && garbage {
                    characters += 1;
                }
            }
        }
        cancel = false;
    }
    return characters;
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        assert_eq!(1, super::part1("{}"));
        assert_eq!(6, super::part1("{{{}}}"));
        assert_eq!(5, super::part1("{{},{}}"));
        assert_eq!(16, super::part1("{{{},{},{{}}}}"));
        assert_eq!(1, super::part1("{<a>,<a>,<a>,<a>}"));
        assert_eq!(9, super::part1("{{<ab>},{<ab>},{<ab>},{<ab>}}"));
        assert_eq!(9, super::part1("{{<!!>},{<!!>},{<!!>},{<!!>}}"));
        assert_eq!(3, super::part1("{{<a!>},{<a!>},{<a!>},{<ab>}}"));
    }

    #[test]
    fn part2() {
        assert_eq!(0, super::part2("<>"));
        assert_eq!(17, super::part2("<random characters>"));
        assert_eq!(3, super::part2("<<<<>"));
        assert_eq!(2, super::part2("<{!>}>"));
        assert_eq!(0, super::part2("<!!>"));
        assert_eq!(0, super::part2("<!!!>>"));
        assert_eq!(10, super::part2("<{o\"i!a,<{i<a>"));
    }
}