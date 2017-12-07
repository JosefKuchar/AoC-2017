use std::fs::File;
use std::io::prelude::*;
use std::error::Error;
use std::collections::HashMap;

fn load_input() -> Result<String, Box<Error>> {
    let mut file = File::open("src/day7/input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    return Ok(contents);
}


pub fn solve() {
    let input = load_input().unwrap();
    let solution1 = part1(&input);
    println!("{}", solution1);
}

fn part1(input: &str) -> String {
    let mut programs: HashMap<String, usize> = HashMap::new();
    for line in input.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        programs.entry(parts[0].to_string()).or_insert(0);
        if parts.len() <= 1 {
            continue;
        }  
        if parts.len() >= 3 {
            for i in 3..parts.len() {
                let mut key = parts[i].to_string();
                key = key.replace(",", "");
                programs.entry(key.clone()).or_insert(0);
                if let Some(references) = programs.get_mut(&key) {
                    *references += 1;
                }
            }
        }
    }



    let (key, _) = programs.into_iter().find(|&(_, value)| value == 0).unwrap();
    return key
}

struct Program {
    parent: String,
    weight: usize
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        assert_eq!("tknk", super::part1("pbga (66)\n\
                                        xhth (57)\n\
                                        ebii (61)\n\
                                        havc (66)\n\
                                        ktlj (57)\n\
                                        fwft (72) -> ktlj, cntj, xhth\n\
                                        qoyq (66)\n\
                                        padx (45) -> pbga, havc, qoyq\n\
                                        tknk (41) -> ugml, padx, fwft\n\
                                        jptl (61)\n\
                                        ugml (68) -> gyxo, ebii, jptl\n\
                                        gyxo (61)\n\
                                        cntj (57)"));
    }
}