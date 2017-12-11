use std::fs::File;
use std::io::prelude::*;
use std::error::Error;

fn load_input() -> Result<String, Box<Error>> {
    let mut file = File::open("src/day11/input.txt")?;
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
    return input.split(',')
        .map(|x| get_coordinate(x))
        .fold(Point::new(0,0,0), |acc, x| acc.add(x))
        .distance();
}

fn get_coordinate(direction: &str) -> Point {
    return match direction {
        "n" => Point::new(0, 1, -1),
        "ne" => Point::new(1, 0, -1),
        "se" => Point::new(1, -1, 0),
        "s" => Point::new(0, -1, 1),
        "sw" => Point::new(-1, 0, 1),
        "nw" => Point::new(-1, 1, 0),
        _ => unimplemented!()
    }
}

struct Point {
    x: isize,
    y: isize,
    z: isize
}

impl Point {
    fn new(x: isize, y: isize, z: isize) -> Point {
        return Point {
            x: x,
            y: y,
            z: z
        }
    }

    fn add(&self, point: Point) -> Point {
        return Point {
            x: self.x + point.x,
            y: self.y + point.y,
            z: self.z + point.z
        }
    }

    fn distance(&self) -> usize{
        return (self.x.abs() + self.y.abs() + self.z.abs()) as usize / 2;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        assert_eq!(3, super::part1("ne,ne,ne"));
        assert_eq!(0, super::part1("ne,ne,sw,sw"));
        assert_eq!(2, super::part1("ne,ne,s,s"));
        assert_eq!(3, super::part1("se,sw,se,sw,sw"));        
    }
}