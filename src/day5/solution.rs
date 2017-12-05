use std::fs::File;
use std::io::prelude::*;
use std::error::Error;

fn load_input() -> Result<String, Box<Error>> {
    let mut file = File::open("src/day5/input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    return Ok(contents);
}

pub fn solve() {
    let contents = load_input().unwrap();
    let solution1 = part1(&mut contents.split('\n').map(|x| x.parse().unwrap()).collect());
    let solution2 = part2(&mut contents.split('\n').map(|x| x.parse().unwrap()).collect());
    println!("{} {}", solution1, solution2);
}

fn part1(instructions: &mut Vec<isize>) -> usize {
    let mut steps = 0;
    let mut index: isize = 0;
    let mut prev_index = 0;

    loop {
        steps += 1;
        index += instructions[index as usize];

        if index < 0 || index as usize >= instructions.len() {
            break;
        }

        instructions[prev_index] += 1; 
        prev_index = index as usize;
    }

    return steps;
}

fn part2(instructions: &mut Vec<isize>) -> usize {
    let mut steps = 0;
    let mut index: isize = 0;
    let mut prev_index = 0;

    loop {
        steps += 1;
        index += instructions[index as usize];

        if index < 0 || index as usize >= instructions.len() {
            break;
        }

        if instructions[prev_index] >= 3 {
            instructions[prev_index] -= 1;
        } else {
            instructions[prev_index] += 1;
        }
        
        prev_index = index as usize;
    }

    return steps;
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        assert_eq!(5, super::part1(&mut vec![0, 3, 0, 1, -3]));
    }

    #[test]
    fn part2() {
        assert_eq!(10, super::part2(&mut vec![0, 3, 0, 1, -3]));
    }
}