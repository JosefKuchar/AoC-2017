use std::fs::File;
use std::io::prelude::*;
use std::error::Error;
use std::collections::HashMap;
use std::collections::HashSet;

fn load_input() -> Result<String, Box<Error>> {
    let mut file = File::open("src/day7/input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    return Ok(contents);
}

pub fn solve() {
    let input = load_input().unwrap();
    let solution1 = part1(&input);
    let solution2 = part2(&input);
    println!("{} {}", solution1, solution2);
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

fn part2(input: &str) -> usize {
    let mut children_set: HashSet<String> = HashSet::new();
    let mut programs: HashMap<String, Program> = HashMap::new();
    for line in input.lines() {
        if line.contains("->") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            let children: Vec<String> = parts[3..].iter().map(|x| x.replace(",", "")).collect();
            for child in &children {
                children_set.insert(child.clone());
            }
            let program = Program {
                weight: parts[1].replace("(", "").replace(")", "").parse().unwrap(),
                children: children
            };
            programs.insert(parts[0].into(), program);
        } else {
            let parts: Vec<&str> = line.split_whitespace().collect();
            let program = Program {
                weight: parts[1].replace("(", "").replace(")", "").parse().unwrap(),
                children: Vec::new()
            };
            programs.insert(parts[0].into(), program);
        }
    }

    let mut root = None;
    for (name, _) in &programs {
        if !children_set.contains(name) {
            root = Some(name.clone());
        }
    }
    find_error(&root.unwrap(), &programs);

    return 0;
}

struct Program {
    weight: u32,
    children: Vec<String>,
}

fn calculate_weight(name: &str, nodes: &HashMap<String, Program>) -> u32 {
    let node = nodes.get(name).unwrap();
    let mut weight = node.weight;
    for child in &node.children {
        weight += calculate_weight(child, &nodes);
    }
    return weight;
}

fn find_error(name: &str, nodes: &HashMap<String, Program>) {
    let node = nodes.get(name).unwrap();
    let weights: Vec<u32> = node.children
        .iter()
        .map(|x| calculate_weight(x, &nodes))
        .collect();
    for w in &weights {
        if *w != weights[0] {
            println!("Error {} != {}", w, weights[0]);
            println!(
            "{} ({}) -> {:?}",
            name,
            node.weight,
            node.children.iter().zip(weights.clone()).collect::<Vec<_>>()
        );
            break;
        }
    }
    
    for child in &node.children {
        find_error(child, &nodes);
    }
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