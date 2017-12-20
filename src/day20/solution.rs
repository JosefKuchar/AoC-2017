use std::fs::File;
use std::io::prelude::*;
use std::error::Error;
use std::collections::HashSet;

fn load_input() -> Result<String, Box<Error>> {
    let mut file = File::open("src/day20/input.txt")?;
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
    let mut particles: Vec<Particle> = input.lines().map(|line| {
        let values: Vec<Vec<isize>> = line.split(", ").map(|value| {
            let start = value.find("<").unwrap() + 1;
            let end = value.find(">").unwrap();
            return value[start..end].trim().split(',').map(|x| x.parse().unwrap()).collect();
        }).collect();
        return Particle {
            position: Point::new(values[0][0], values[0][1], values[0][2]),
            velocity: Point::new(values[1][0], values[1][1], values[1][2]),
            acceleration: Point::new(values[2][0], values[2][1], values[2][2])
        }
    }).collect();
    
    let mut min_index = 0;
    let mut min_distance = usize::max_value();
    for (index, particle) in particles.iter().enumerate() {
        if distance(&particle.acceleration) < min_distance {
            min_distance = distance(&particle.acceleration);
            min_index = index;
        }
    }

    for _ in 0..100 {
        for particle in particles.iter_mut() {
            particle.update();
        }

        let mut duplicates: Vec<Point> = Vec::new();
        particles.dedup_by(|a, b| {
            if a.position.equal(&b.position) {
                duplicates.push(a.position);
                return true
            } else {
                return false
            }
        });
        duplicates.dedup();
        for i in (0..particles.len()).rev() {
            for duplicate in &duplicates {
                if particles[i].position.equal(duplicate) {
                    particles.remove(i);
                    break;
                }
            }
        }
    }

    return (min_index, particles.len());
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Point {
    x: isize,
    y: isize,
    z: isize
}

impl Point {
    fn new(x: isize, y: isize, z: isize) -> Point {
        Point {
            x: x,
            y: y,
            z: z
        }
    }

    fn add(&mut self, point: &Point) {
        self.x += point.x;
        self.y += point.y;
        self.z += point.z;
    }

    fn equal(&self, point: &Point) -> bool {
        return self.x == point.x && self.y == point.y && self.z == point.z
    }
}

fn distance(point: &Point) -> usize {
    return (point.x.abs() + point.y.abs() + point.z.abs()) as usize;
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Particle {
    position: Point,
    velocity: Point,
    acceleration: Point
}

impl Particle {
    fn update(&mut self) {
        self.velocity.add(&self.acceleration);
        self.position.add(&self.velocity);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn solve() {
        assert_eq!((0, 2), super::solution("p=< 3,0,0>, v=< 2,0,0>, a=<-1,0,0>\n\
                                            p=< 4,0,0>, v=< 0,0,0>, a=<-2,0,0>"));
        assert_eq!((0, 1), super::solution("p=<-6,0,0>, v=< 3,0,0>, a=< 0,0,0>\n\
                                            p=<-4,0,0>, v=< 2,0,0>, a=< 0,0,0>\n\
                                            p=<-2,0,0>, v=< 1,0,0>, a=< 0,0,0>\n\
                                            p=< 3,0,0>, v=<-1,0,0>, a=< 0,0,0>"));
    }
}