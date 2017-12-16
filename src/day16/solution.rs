use std::fs::File;
use std::io::prelude::*;
use std::error::Error;

fn load_input() -> Result<String, Box<Error>> {
    let mut file = File::open("src/day16/input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    return Ok(contents);
}

pub fn solve() {
    let contents = load_input().unwrap();
    let part1 = part1(&contents, "abcdefghijklmnop".to_owned());
    let part2 = part2(&contents);
    println!("{} {}", part1, part2);
}

fn part2(input: &str) -> String {
    let mut string = "abcdefghijklmnop".to_owned();
    let mut index = 1;
    loop {
        string = part1(input, string);
        if string == "abcdefghijklmnop" {
            break;
        }
        index += 1;
    }
    for i in 0..(1_000_000_000 % index) {
        string = part1(input, string);
    }
    return string;
}

fn part1(input: &str, initial: String) -> String {
    let mut string = initial;
    for action in input.split(',') {
        let code = action.chars().nth(0).unwrap();
        match code {
            's' => {
                string = spin(string.clone(), action.get(1..).unwrap().parse::<usize>().unwrap());
            },
            'x' => {
                let parts: Vec<&str> = action.get(1..).unwrap().split('/').collect();
                string = exchange(string.clone(), parts[0].parse().unwrap(), parts[1].parse().unwrap());
            }
            'p' => {
                string = partner(string.clone(), action.chars().nth(1).unwrap(), action.chars().nth(3).unwrap());
            },
            _ => unimplemented!()
        }
    }
    return string;
}

fn spin(string: String, moves: usize) -> String {
    let mut new_string = String::with_capacity(string.len());
    for i in (string.len() - moves)..(2 * string.len() - moves) {
        new_string.push(string.chars().nth(i % string.len()).unwrap())
    }
    return new_string;
}

fn exchange(string: String, a: usize, b: usize) -> String {
    let mut new_string = String::with_capacity(string.len());
    for (index, character) in string.chars().enumerate() {
        if index == a {
            new_string.push(string.chars().nth(b).unwrap())
        } else if index == b {
            new_string.push(string.chars().nth(a).unwrap())
        } else {
            new_string.push(character)
        }
    }
    return new_string;
}

fn partner(string: String, a: char, b: char) -> String {
    return exchange(string.clone(), string.find(a).unwrap(), string.find(b).unwrap());
}

#[cfg(test)]
mod tests {
    #[test]
    fn spin() {
        assert_eq!("cdeab", super::spin("abcde".to_owned(), 3));
        assert_eq!("eabcd", super::spin("abcde".to_owned(), 1));
    }

    #[test]
    fn exchange() {
        assert_eq!("eabdc", super::exchange("eabcd".to_owned(), 3, 4));
    }

    #[test]
    fn partner() {
        assert_eq!("baedc", super::partner("eabdc".to_owned(), 'e', 'b'));
    }
}