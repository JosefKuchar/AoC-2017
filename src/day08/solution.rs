use std::fs::File;
use std::io::prelude::*;
use std::error::Error;
use std::collections::HashMap;

fn load_input() -> Result<String, Box<Error>> {
    let mut file = File::open("src/day8/input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    return Ok(contents);
}

pub fn solve() {
    let (part1, part2) = process(&load_input().unwrap());
    println!("{} {}", part1, part2);
}

fn process(input: &str) -> (isize, isize) {
    let mut variables: HashMap<&str, isize> = HashMap::new();
    let mut max_value = 0;

    for line in input.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        variables.entry(parts[0]).or_insert(0);
        variables.entry(parts[4]).or_insert(0);
        
        let register_y = variables.get(parts[4]).unwrap().clone();
        let register_x = variables.get_mut(parts[0]).unwrap();
        let value_x: isize = parts[2].parse().unwrap();
        let value_y: isize = parts[6].parse().unwrap();

        let result = match parts[5] {
            ">" => register_y > value_y,
            "<" => register_y < value_y,
            ">=" => register_y >= value_y,
            "<=" => register_y <= value_y,
            "!=" => register_y != value_y,
            "==" => register_y == value_y,
            _ => unimplemented!()
        };

        if !result {
            continue;
        }
        
        *register_x += if parts[1] == "inc" { 
            value_x
        } else { 
            -value_x
        };

        if *register_x > max_value {
            max_value = *register_x;
        }
    }

    return (*variables.values().max().unwrap(), max_value);
}

#[cfg(test)]
mod tests {
    #[test]
    fn solve() {
        assert_eq!((1, 10), super::process("b inc 5 if a > 1\n\
                                            a inc 1 if b < 5 (57)\n\
                                            c dec -10 if a >= 1\n\
                                            c inc -20 if c == 10"));
    }
}