pub fn solve() {
    let part1 = part1(354);
    let part2 = part2(354);
    println!("{} {}", part1, part2);
}

fn part1(steps: usize) -> usize {
    let mut buffer: Vec<usize> = vec![0];
    let mut position = 0;
    for i in 1..2018 {
        position += steps + 1;
        position %= buffer.len();
        buffer.insert(position, i);
    }
    return buffer[(position + 1) % buffer.len()];
}

fn part2(steps: usize) -> usize {
    let mut index = 0;
    let mut zero_position = 0;
    let mut neighbor = 0;
    let mut buffer = 1;
    for i in 1..50000000 {
        index = ((steps + index) % buffer) + 1;
        buffer += 1;

        if index <= zero_position {
            zero_position += 1;
        }

        if index == zero_position + 1 {
            neighbor = i;
        }
    }
    return neighbor;
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        assert_eq!(638, super::part1(3));
    }
}