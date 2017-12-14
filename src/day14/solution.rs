use std::usize;

pub fn solve() {
    let part1 = part1("ffayrhll");
    let part2 = part2("ffayrhll");
    println!("{} {}", part1, part2);
}

fn part1(input: &str) -> usize {
    let mut sum = 0;
    for index in 0..128 {
        sum += knot_hash(&format!("{}-{}", input, index)).iter().filter(|x| **x).count();
    }
    return sum;
}

fn part2(input: &str) -> usize {
    let mut grid: Vec<Vec<bool>> = Vec::new();
    for index in 0..128 {
        grid.push(knot_hash(&format!("{}-{}", input, index)));
    }

    let mut group = 0;
    for x in 0..128 {
        for y in 0..128 {
            if grid[x][y] {
                let mut queue: Vec<Point> = Vec::new();
                queue.push(Point::new(x as isize, y as isize));
                let directions = [Point::new(1, 0), Point::new(-1, 0), Point::new(0, 1), Point::new(0, -1)];

                while !queue.is_empty() {
                    let element = queue.remove(0);
                    grid[element.x as usize][element.y as usize] = false;

                    for direction in &directions {
                        let mut new_element: Point = element.clone();
                        new_element.add(direction);
                        if new_element.in_bounds() && grid[new_element.x as usize][new_element.y as usize] {
                            queue.push(new_element);
                        }
                    }
                }
                group += 1;
            }
        }
    }
    return group;
}

fn knot_hash(input: &str) -> Vec<bool> {
    let mut lengths: Vec<usize> = input.chars().map(|x| x as usize).collect();
    lengths.append(&mut vec![17, 31, 73, 47, 23]);
    let mut skip_size = 0;
    let mut position = 0;
    let mut numbers = (0..256).collect::<Vec<_>>();
    for _ in 0..64 {
        for length in &lengths  {
            for i in 0..length/2 {
                numbers.swap((position + i) % 256, (position + length - 1 - i) % 256);
            }

            position += length + skip_size;
            position %= 256;
            skip_size += 1;
        }
    }
    let mut hash = String::new();
    
    for block in numbers.chunks(16) {
        hash += &format!("{:02x}", block.iter().fold(0, |acc, &num| acc ^ num));
    }
    
    let binary: String = hash
        .clone()
        .chars()
        .map(|x| format!("{:04b}", usize::from_str_radix(&x.to_string(), 16).unwrap()))
        .collect();

    return binary.chars().map(|x| x == '1').collect();
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

    fn in_bounds(&self) -> bool {
        return self.x >= 0 && self.x < 128 && self.y >= 0 && self.y < 128;
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
        assert_eq!(8108, super::part1("flqrgnkx"));
    }

    #[test]
    fn part2() {
        assert_eq!(1242, super::part2("flqrgnkx"));
    }
}