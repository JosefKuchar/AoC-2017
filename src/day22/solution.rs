use std::fs::File;
use std::io::prelude::*;
use std::error::Error;
use std::collections::VecDeque;

fn load_input() -> Result<String, Box<Error>> {
    let mut file = File::open("src/day22/input.txt")?;
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

fn part1(input: &str) -> usize {
    let DIRECTIONS = [Point::new(0, -1), Point::new(1, 0), Point::new(0, 1), Point::new(-1, 0)];
    let mut direction = 0;
    let mut infections = 0;
    let mut grid: VecDeque<VecDeque<bool>> = input.lines().map(|line| line.chars().map(|x| match x {
        '.' => false,
        '#' => true,
        _ => panic!()
    }).collect()).collect();
    let mut width = grid[0].len();
    let mut height = grid.len();
    let mut position = Point::new(width as isize / 2, height as isize / 2);

    for _ in 0..10000 {
        let infected = grid[position.y as usize][position.x as usize];
        direction += match infected {
            true => 1,
            false => 3
        };
        direction %= 4;
        if !infected {
            infections += 1;
        }
        grid[position.y as usize][position.x as usize] = !infected;
        position.add(&DIRECTIONS[direction]);

        if position.x < 0 {
            for row in grid.iter_mut() {
                row.push_front(false);
            }
            position.x = 0;
            width += 1;
        } else if position.x >= width as isize {
            for row in grid.iter_mut() {
                row.push_back(false);
            }
            width += 1;
            position.x = width as isize - 1;
        } else if position.y < 0 {
            let mut row = VecDeque::new();
            row.extend(vec![false; width]);
            grid.push_front(row);
            position.y = 0;
            height += 1;
        } else if position.y >= height as isize {
            let mut row = VecDeque::new();
            row.extend(vec![false; width]);
            grid.push_back(row);
            height += 1;
            position.y = height as isize - 1;
        }
    }

    return infections;
}

fn part2(input: &str) -> usize {
    let DIRECTIONS = [Point::new(0, -1), Point::new(1, 0), Point::new(0, 1), Point::new(-1, 0)];
    let mut direction = 0;
    let mut infections = 0;
    let mut grid: VecDeque<VecDeque<usize>> = input.lines().map(|line| line.chars().map(|x| match x {
        '.' => 0,
        '#' => 2,
        _ => panic!()
    }).collect()).collect();
    let mut width = grid[0].len();
    let mut height = grid.len();
    let mut position = Point::new(width as isize / 2, height as isize / 2);

    for _ in 0..10000000 {
        let status = grid[position.y as usize][position.x as usize];
        direction += match status {
            0 => 3,
            1 => 0,
            2 => 1,
            3 => 2,
            _ => panic!()
        };
        direction %= 4;
        if status == 1 {
            infections += 1;
        }
        grid[position.y as usize][position.x as usize] += 1;
        grid[position.y as usize][position.x as usize] %= 4;
        position.add(&DIRECTIONS[direction]);

        if position.x < 0 {
            for row in grid.iter_mut() {
                row.push_front(0);
            }
            position.x = 0;
            width += 1;
        } else if position.x >= width as isize {
            for row in grid.iter_mut() {
                row.push_back(0);
            }
            width += 1;
            position.x = width as isize - 1;
        } else if position.y < 0 {
            let mut row = VecDeque::new();
            row.extend(vec![0; width]);
            grid.push_front(row);
            position.y = 0;
            height += 1;
        } else if position.y >= height as isize {
            let mut row = VecDeque::new();
            row.extend(vec![0; width]);
            grid.push_back(row);
            height += 1;
            position.y = height as isize - 1;
        }
    }

    return infections;
}

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
        assert_eq!(5587, super::part1("..#\n#..\n..."));
    }

    #[test]
    fn part2() {
        assert_eq!(2511944, super::part2("..#\n#..\n..."));
    }
}