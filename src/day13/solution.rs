use std::fs::File;
use std::io::prelude::*;
use std::error::Error;

fn load_input() -> Result<String, Box<Error>> {
    let mut file = File::open("src/day13/input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    return Ok(contents);
}

pub fn solve() {
    let contents = load_input().unwrap();
    let (part1, part2) = solution(&contents);
    println!("{} {}", part1, part2);
}

fn solution(input: &str) -> (usize, usize) {
    let mut layers: Vec<Layer> = generate_layers(&input);
    let mut current_layer = 0;
    let mut score = 0;

    for (index, layer) in layers.clone().iter().enumerate() {
        if layers[index].range != 0 && layers[index].position == 0 {
            score += index * layers[index].range;
        }
        update_layers(&mut layers);
    }

    let mut delay = 0;
    while layers.iter().filter(|&l| l.range != 0).any(|&l| (l.index + delay) % ((l.range - 1) * 2) == 0) {
        delay += 1;
    }

    return (score, delay);
}


#[derive(Debug, Clone, Copy)]
struct Layer {
    index: usize,
    range: usize,
    direction: bool,
    position: usize
}

impl Layer {
    fn from_str(string: &str) -> Layer {
        let parts: Vec<&str> = string.split(": ").collect();
        return Layer {
            position: 0,
            index: parts[0].parse().unwrap(),
            range: parts[1].parse().unwrap(),
            direction: true
        }
    }
}

fn update_layers(layers: &mut Vec<Layer>) {
    for layer in layers.iter_mut() {
        if layer.range != 0 {
            if layer.direction {
                layer.position += 1;
                if layer.position + 1 == layer.range {
                    layer.direction = false;
                }
            } else {
                layer.position -= 1; 
                if layer.position == 0 {
                    layer.direction = true;
                }
            }
        }   
    }
}

fn generate_layers(input: &str) -> Vec<Layer> {
    let scanners: Vec<Layer> = input.lines().map(|x| Layer::from_str(x)).collect();
    let mut layers: Vec<Layer> = vec![Layer {
        index: 0,
        position: 0,
        range: 0,
        direction: true
    }; scanners.last().unwrap().index + 1];
    for (index, layer) in layers.iter_mut().enumerate() {
        layer.index = index;
    }
    for scanner in &scanners {
        layers[scanner.index].range = scanner.range;
    }
    return layers;
}

#[cfg(test)]
mod tests {
    #[test]
    fn solve() {
        assert_eq!((24, 10), super::solution("0: 3\n\
                                        1: 2\n\
                                        4: 4\n\
                                        6: 4"));
    }
}