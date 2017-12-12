use std::fs::File;
use std::io::prelude::*;
use std::error::Error;

fn load_input() -> Result<String, Box<Error>> {
    let mut file = File::open("src/day01/input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    return Ok(contents);
}

fn char_to_int(character: char) -> usize {
    return character.to_digit(10).expect("Invalid character in input") as usize;
}

fn part1(input: &String) -> usize {
    let mut sum = 0;

    for (index, character) in input.chars().enumerate() {
        let next_index = (index + 1) % input.len();
        if character == input.chars().nth(next_index).unwrap() {
            sum += char_to_int(character);
        }
    }

    return sum;
}

fn part2(input: &String) -> usize {
    let mut sum = 0;

    for (index, character) in input.chars().enumerate() {
        let next_index = (index + input.len() / 2) % input.len();
        if character == input.chars().nth(next_index).unwrap() {
            sum += char_to_int(character);
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
        assert_eq!(3, super::part1(&"1122".to_string()));
        assert_eq!(4, super::part1(&"1111".to_string()));
        assert_eq!(0, super::part1(&"1234".to_string()));
        assert_eq!(9, super::part1(&"91212129".to_string()));                
    }

    #[test]
    fn part2() {
        assert_eq!(6, super::part2(&"1212".to_string()));
        assert_eq!(0, super::part2(&"1221".to_string()));
        assert_eq!(4, super::part2(&"123425".to_string()));
        assert_eq!(12, super::part2(&"123123".to_string()));                
        assert_eq!(4, super::part2(&"12131415".to_string()));                
    }
}