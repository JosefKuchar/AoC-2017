use std::fs::File;
use std::io::prelude::*;
use std::error::Error;

fn load_input() -> Result<String, Box<Error>> {
    let mut file = File::open("src/day10/input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    return Ok(contents);
}

pub fn solve() {
    let contents = load_input().unwrap();
    let solution1 = part1(contents.split(',').map(|x| x.parse().unwrap()).collect(), 256);
    println!("{}", solution1);
}

fn part1(input: Vec<usize>, size: usize) -> usize {
    let mut skip_size = 0;
    let mut position = 0;
    let mut array: Vec<usize> = Vec::new();
    generate_array(&mut array, size);
    for length in &input {
        reverse(&mut array, position, *length + position);
        position += *length + skip_size;
        skip_size += 1;
    }
    return array[0] * array[1];
}

fn part2(input: Vec<usize>) -> String {
    return "asdas".to_string();
}

fn generate_array(array: &mut Vec<usize>, size: usize) {
    for index in 0..size {
        array.push(index);
    }
}

fn reverse(array: &mut Vec<usize>, start: usize, end: usize) {
    let mut temp_array: Vec<usize> = Vec::new();
    let size = array.len();
    for i in start..end {
        temp_array.push(array[i % size]);
    }
    for i in start..end {
        array[i % size] = temp_array.pop().unwrap();
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        assert_eq!(12, super::part1(vec![3, 4, 1, 5], 5));
    }

    #[test]
    fn part2() {
        assert_eq!("33efeb34ea91902bb2f59c9920caa6cd", super::part2("AoC 2017".chars().map(|x| x as usize).collect()));
    }
}