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
    let solution2 = part2(contents.chars().map(|x| x as usize).collect());
    println!("{} {}", solution1, solution2);
}

fn part1(input: Vec<usize>, size: usize) -> usize {
    let mut skip_size = 0;
    let mut position = 0;
    let mut array: Vec<usize> = (0..size).collect::<Vec<_>>();
    for length in &input {
        reverse(&mut array, position, *length + position);
        position += *length + skip_size;
        position %= size;
        skip_size += 1;
    }
    return array[0] * array[1];
}

fn part2(input: Vec<usize>) -> String {
    let mut lengths = input.clone();
    lengths.append(&mut vec![17, 31, 73, 47, 23]);
    let mut skip_size = 0;
    let mut position = 0;
    let mut numbers = (0..256).collect::<Vec<_>>();
    for _ in 0..64 {
        for length in &lengths  {
            for i in 0..length/2 {
                numbers.swap((position + i) % 256, (position + length - 1 - i) % 256);
            }

            position += length + skip_size;
            position %= 256;
            skip_size += 1;
        }
    }
    let mut hash = String::new();
    for block in numbers.chunks(16) {
        hash += &format!("{:02x}", block.iter().fold(0, |acc, &num| acc ^ num));
    }

    return hash;
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
        assert_eq!("a2582a3a0e66e6e86e3812dcb672a272", super::part2(Vec::new()));
        assert_eq!("3efbe78a8d82f29979031a4aa0b16a9d", super::part2("1,2,3".chars().map(|x| x as usize).collect()));
    }
}