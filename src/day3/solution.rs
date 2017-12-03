fn get_size(input: usize) -> usize {
    let mut index = 0;
    loop {
        let square = ((input + index) as f64).sqrt();
        let checker = square.floor();

        if square == checker {
            return square as usize;
        }
        index += 1;
    }
}

fn part1(input: usize) -> usize {
    let size = get_size(input);
    let tiles = size * size;
    let diff = tiles - input;

    if diff <= size - 1 {
        return size / 2 + (diff as isize - size as isize / 2).abs() as usize;
    } else {
        unimplemented!();
    }
}

fn part2(input: usize) -> usize  {
    static DIRECTIONS: [[isize; 2]; 4] = [[1, 0], [0, -1], [-1, 0], [0, 1]];
    let mut grid = [[0usize; 200]; 200];
    let mut size = 1;
    let mut counter = 0;
    let mut steps = 0;
    let mut x = 100;
    let mut y = 100;
    grid[100][100] = 1;
    let mut direction = 0;

    loop {
        x = (x as isize + DIRECTIONS[direction][0]) as usize;
        y = (y as isize + DIRECTIONS[direction][1]) as usize;
        steps += 1;

        for i in 0..3 {
            for j in 0..3 {
                if i == 1 && j == 1 {
                    continue;
                }
                grid[x][y] += grid[x + i - 1][y + j - 1];
            }
        }

        if steps == size {
            direction += 1;
            direction %= 4;
            steps = 0;
            counter += 1;

            if counter == 2 {
                counter = 0;
                size += 1;
            }
        }

        if grid[x][y] >= input {
            return grid[x][y];
        }
    }
}

pub fn solve() {
    let solution1 = part1(265149);
    let solution2 = part2(265149);
    println!("{} {}", solution1, solution2);
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        //assert_eq!(0, super::part1(1));
        //assert_eq!(3, super::part1(12));
        assert_eq!(2, super::part1(23));
        //assert_eq!(31, super::part1(1024));
    }

    #[test]
    fn part2() {
        assert_eq!(23, super::part2(21));
    }

    #[test]
    fn get_size() {
        assert_eq!(5, super::get_size(23));
        assert_eq!(4, super::get_size(14));
    }
}