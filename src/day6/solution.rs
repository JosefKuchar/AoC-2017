use std::fs::File;
use std::io::prelude::*;
use std::error::Error;

fn load_input() -> Result<String, Box<Error>> {
    let mut file = File::open("src/day6/input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    return Ok(contents);
}

pub fn solve() {
    let contents = load_input().unwrap();
    let solution1 = part1(&mut contents.split("\t").map(|x| x.parse().unwrap()).collect());
    let solution2 = part2(&mut contents.split("\t").map(|x| x.parse().unwrap()).collect());
    println!("{} {}", solution1, solution2)
}

fn reallocate(banks: &mut Vec<usize>) {
    let mut blocks = *banks.iter().max().unwrap();
    let mut index = banks.iter().position(|x| *x == blocks).unwrap();
    banks[index] = 0;
    while blocks > 0 {
        index += 1;
        index %= banks.len();
        banks[index] += 1;
        blocks -= 1;
    }
}

fn part1(banks: &mut Vec<usize>) -> usize {
    let mut reallocations: Vec<Vec<usize>> = Vec::new();
    let mut cycle = 0;

    loop {
        reallocate(banks);
        cycle += 1;    
        for reallocation in &reallocations {
            if reallocation == banks {
                return cycle;
            }
        }
        reallocations.push(banks.clone());

    }
}

fn part2(banks: &mut Vec<usize>) -> usize {
    let mut reallocations: Vec<Vec<usize>> = Vec::new();
    let mut cycle = 0;

    loop {
        reallocate(banks);
        cycle += 1;    
        for (index, reallocation) in reallocations.iter().enumerate() {
            if reallocation == banks {
                return cycle - index - 1;
            }
        }
        reallocations.push(banks.clone());

    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        assert_eq!(5, super::part1(&mut vec![0, 2, 7, 0]));
    }

    #[test]
    fn part2() {
        assert_eq!(4, super::part2(&mut vec![0, 2, 7, 0]));
    }
}