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
    let part2 = part2(&contents);
    println!("{} {}", part1, part2);
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

fn part2(input: &str) -> usize{
    let mut program_0 = Program::new(0, input);
    let mut program_1 = Program::new(1, input);
    program_0.run(&mut Vec::new());
    for i in 0..300 {
        program_1.run(&mut program_0.queue);
        program_0.run(&mut program_1.queue);
    }

    return program_1.sends;
}

struct Program {
    registers: HashMap<String, isize>,
    queue: Vec<isize>,
    code: String,
    index: usize,
    sends: usize
}

impl Program {
    fn new(id: isize, code: &str) -> Program {
        let mut registers = HashMap::new();
        registers.insert("p".to_owned(), id);
        return Program {
            registers: registers,
            queue: Vec::new(),
            code: code.to_owned(),
            index: 0,
            sends: 0
        }
    }

    fn run(&mut self, partner_queue: &mut Vec<isize>) {
        loop {
            let lines: Vec<&str> = self.code.lines().collect();
            let parts: Vec<&str> = lines[self.index].split_whitespace().collect();

            let mut y = 0;       
            if let Some(register) = parts.get(2) {
                let number = register.parse::<isize>();
                y = match number {
                    Ok(value) => value,
                    Err(_) => *(self.registers.entry(register.to_owned().to_owned()).or_insert(0))
                }
            }

            let x = self.registers.entry(parts[1].to_owned()).or_insert(0);
            match parts[0] {
                "snd" => {
                    self.queue.push(*x);
                    self.sends += 1;
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
                    if partner_queue.len() > 0 {
                        *x = partner_queue.remove(0);
                    } else {
                        break;
                    }
                },
                "jgz" => {
                    if *x > 0 || parts[1] == "1" {
                        self.index = (self.index as isize + y - 1) as usize;
                    }
                },
                _ => unimplemented!()
            }
            self.index += 1;
        }
    }
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