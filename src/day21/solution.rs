use std::fs::File;
use std::io::prelude::*;
use std::error::Error;

fn load_input() -> Result<String, Box<Error>> {
    let mut file = File::open("src/day21/input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    return Ok(contents);
}

pub fn solve() {
    let contents = load_input().unwrap();
    let part1 = solution(&contents, 5);
    let part2 = solution(&contents, 18);
    println!("{} {}", part1, part2);
}

fn solution(input: &str, iterations: usize) -> usize {
    let mut pattern = vec![vec![false, true, false],
                            vec![false, false, true],
                            vec![true, true, true]];
    let rules = parse_rules(input);

    for _ in 0..iterations {
        let mut patterns = split_pattern(&pattern);

        for i in patterns.iter_mut() {
            for j in i {
                *j = match_rule(j, &rules);
            }
        }
        
        pattern = join_patterns(&patterns);
    }
    
    let mut sum = 0;
    for row in &pattern {
        sum += row.iter().filter(|x| **x).count();
    }
    return sum;
}

fn match_rule(pattern: &Vec<Vec<bool>>, rules: &Vec<Rule>) -> Vec<Vec<bool>> {
    for rule in rules {
        let mut cloned = rule.clone().input;
        for _ in 0..4 {
            cloned = rotate(&cloned);
            if cloned == *pattern || flip(&cloned) == *pattern {
                return rule.clone().output;
            }
        }    
    }
    unimplemented!();   
}

fn parse_rules(rules: &str) -> Vec<Rule> {
    return rules.lines().map(|line| {
        let parts: Vec<&str> = line.split(" => ").collect();
        let input = parts[0].split('/').map(|row| row.chars().map(|character| match character {
            '#' => true,
            '.' => false,
            _ => unimplemented!()
        }).collect()).collect();
        let output = parts[1].split('/').map(|row| row.chars().map(|character| match character {
            '#' => true,
            '.' => false,
            _ => unimplemented!()
        }).collect()).collect();
        return Rule {
            input: input,
            output: output
        }
    }).collect();
}

fn rotate(array: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let mut rotated: Vec<Vec<bool>> = Vec::new();
    for x in 0..array.len() {
        rotated.push(Vec::new());
        for y in 0..array[0].len() {
            rotated[x].push(array[array.len() - y - 1][x]);
        }
    }
    return rotated;
}

fn flip(array: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let mut flipped: Vec<Vec<bool>> = Vec::new();
    for x in 0..array.len() {
        flipped.push(Vec::new());
        for y in 0..array[0].len() {
            flipped[x].push(array[x][array.len() - y - 1]);
        }
    }
    return flipped;
}

fn split_pattern(pattern: &Vec<Vec<bool>>) -> Vec<Vec<Vec<Vec<bool>>>> {
    let size = pattern.len();
    let mut block = 0;
    let mut blocks = 0;
    let mut patterns: Vec<Vec<Vec<Vec<bool>>>> = Vec::new();
    if size % 2 == 0 {
        block = 2;
        blocks = size / 2;
    } else if size % 3 == 0 {
        block = 3;
        blocks = size / 3;
    } else {
        unimplemented!();
    }
    
    for x in 0..blocks {
        patterns.push(Vec::new());
        for y in 0..blocks {
            patterns[x].push(Vec::new());
            for i in 0..block {
                patterns[x][y].push(Vec::new());
                for j in 0..block {
                    patterns[x][y][i].push(pattern[x * block + i][y * block + j]);
                } 
            }
        }
    }

    return patterns;
}

fn join_patterns(patterns: &Vec<Vec<Vec<Vec<bool>>>>) -> Vec<Vec<bool>> {
    let mut pattern: Vec<Vec<bool>> = Vec::new();
    let blocks = patterns.len();
    let block = patterns[0][0].len();
    for _ in 0..(blocks * block) {
        pattern.push(Vec::new());
    }
    for x in 0..blocks {
        for y in 0..blocks {
            for i in 0..block {
                for j in 0..block {
                    pattern[x * block + i].push(patterns[x][y][i][j]);
                }
            }
        }
    }
    return pattern;
}

#[derive(Debug, PartialEq, Clone)]
struct Rule {
    input: Vec<Vec<bool>>,
    output: Vec<Vec<bool>>
}

#[cfg(test)]
mod tests {
    #[test]
    fn solve() {
        assert_eq!(12, super::solution("../.# => ##./#../...\n.#./..#/### => #..#/..../..../#..#", 2));
    }
}