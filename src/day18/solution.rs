use std::fs::File;
use std::io::prelude::*;
use std::error::Error;
use std::collections::HashMap;

fn load_input() -> Result<String, Box<Error>> {
    let mut file = File::open("src/day18/input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    return Ok(contents);
}

pub fn solve() {
    let contents = load_input().unwrap();
    let part1 = part1(&contents);
    println!("{}", part1);
}

fn part1(input: &str) -> isize {
    let mut registers: HashMap<&str, isize> = HashMap::new();
    let mut last_frequency = 0;
    let lines: Vec<&str> = input.split('\n').collect();
    let mut index: isize = 0;
    loop {
        let parts: Vec<&str> = lines[index as usize].split_whitespace().collect();

        let mut y = 0;       
        if let Some(register) = parts.get(2) {
            let number = register.parse::<isize>();
            y = match number {
                Ok(value) => value,
                Err(_) => *(registers.entry(register).or_insert(0))
            }
        }

        let mut x = registers.entry(parts[1]).or_insert(0);
        match parts[0] {
            "snd" => {
                last_frequency = *x;
            },
            "set" => {
                *x = y
            },
            "add" => {
                *x += y;
            },
            "mul" => {
                *x *= y; 
            },
            "mod" => {
                *x %= y;
            },
            "rcv" => {
                if *x != 0 {
                    return last_frequency;
                }
            },
            "jgz" => {
                if *x > 0 {
                    index += y - 1;
                }
            },
            _ => unimplemented!()
        }
        index += 1;
    }

    return last_frequency;
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        assert_eq!(4, super::part1("set a 1\n\
                                    add a 2\n\
                                    mul a a\n\
                                    mod a 5\n\
                                    snd a\n\
                                    set a 0\n\
                                    rcv a\n\
                                    jgz a -1\n\
                                    set a 1\n\
                                    jgz a -2"));
    }
}