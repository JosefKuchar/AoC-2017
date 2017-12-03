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
    get_coordinates(265149);
    unimplemented!();
}

pub fn solve() {
    let solution1 = part1(265149);
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