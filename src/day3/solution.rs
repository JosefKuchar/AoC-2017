fn get_coordinates(input: usize) {
    let size = get_size(input);
    let tiles = size * size;
    let diff = tiles - input;

    if diff <= size - 1 {
        println!("{}", size / 2);
        println!("{}", diff as isize - size as isize / 2)
    }
}

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
    get_coordinates(input);
    //unimplemented!();
    return 0;
}

fn part2() {
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

        if grid[x][y] >= 265149 {
            println!("{}", grid[x][y]);
            break;
        }

        //println!("{}", grid[x][y]);
    }

}

pub fn solve() {
    let solution1 = part1(265149);
    part2();
    println!("{}", solution1);
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        assert_eq!(0, super::part1(1));
        assert_eq!(3, super::part1(12));
        assert_eq!(2, super::part1(23));
        assert_eq!(31, super::part1(1024));
    }

    #[test]
    fn get_size() {
        assert_eq!(5, super::get_size(23));
        assert_eq!(4, super::get_size(14));
    }
}