use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::error::Error;

fn load_input() -> Result<String, Box<Error>> {
    let mut file = File::open("src/day12/input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    return Ok(contents);
}

pub fn solve() {
    let contents = load_input().unwrap();
    let solution1 = part1(&contents);
    println!("{}", solution1);
}

fn part1(input: &str) -> usize {
    let mut numbers: Vec<HashSet<usize>> = input.lines().map(|x| parse_line(x)).collect();
    let mut group: HashSet<usize> = HashSet::new();
    group.insert(0);

    loop {
        let mut global_contains = false;
        for index in (0..numbers.len()).rev() {
            let mut contains = false;
            for number in numbers[index].iter() {
                if group.contains(number) {
                    contains = true;
                    global_contains = true;
                    break;
                }
            }
            if contains {
                for number in &numbers[index] {
                    group.insert(*number);
                }
                numbers.remove(index);
            }
        }
        if !global_contains {
            break;
        } 
    }

    return group.len();
}

fn parse_line(line: &str) -> HashSet<usize>  {
    let parts: Vec<&str> = line.split(" <-> ").collect();
    let mut numbers: Vec<usize> = parts[1].split(", ").map(|x| x.parse().unwrap()).collect();
    numbers.push(parts[0].parse().unwrap());
    return numbers.iter().cloned().collect();
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        assert_eq!(6, super::part1("0 <-> 2\n\
                                    1 <-> 1\n\
                                    2 <-> 0, 3, 4\n\
                                    3 <-> 2, 4\n\
                                    4 <-> 2, 3, 6\n\
                                    5 <-> 6\n\
                                    6 <-> 4, 5"));
    } 
}