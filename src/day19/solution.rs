use std::fs::File;
use std::io::prelude::*;
use std::error::Error;

fn load_input() -> Result<String, Box<Error>> {
    let mut file = File::open("src/day19/input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    return Ok(contents);
}

pub fn solve() {
    let contents = load_input().unwrap();
    let part1 = part1(&contents);
    println!("{}", part1);
}

fn part1(input: &str) -> String {
    let grid: Vec<Vec<char>> = input.lines().map(|x| x.chars().collect()).collect();
    let mut letters = String::new();
    let mut position = Point::new(0, 0);
    let mut direction = Point::new(0, 1);
    // Find start
    for (index, character) in grid[0].iter().enumerate() {
        if *character == '|' {
            position = Point::new(index as isize, 0);
        }
    }

    loop {
        match grid[position.y as usize][position.x as usize] {
            ' ' => {
                break;
            },
            '+' => {
                // Vertical
                if direction.x == 0 {
                    direction.y = 0;
                    if grid[position.y as usize][position.x as usize - 1] == ' ' {
                        direction.x = 1;
                    } else {
                        direction.x = -1;
                    }
                // Horizontal
                } else {
                    direction.x = 0;
                    if grid[position.y as usize - 1][position.x as usize] == ' ' {
                        direction.y = 1;
                    } else {
                        direction.y = -1;
                    }
                }
            },
            '|' => {},
            '-' => {},
            _ => {
                letters.push(grid[position.y as usize][position.x as usize])
            }
        }
        position.add(&direction);
    }

    return letters;
}

#[derive(Debug, Clone, Copy)]
struct Point {
    x: isize, 
    y: isize
}

impl Point {
    fn new(x: isize, y: isize) -> Point {
        return Point {
            x: x,
            y: y
        }
    }

    fn add(&mut self, point: &Point) {
        self.x += point.x;
        self.y += point.y;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        assert_eq!("ABCDEF", super::part1("     |          \n     |  +--+    \n     A  |  C    \n F---|----E|--+ \n     |  |  |  D \n     +B-+  +--+ \n                "));
    }
}