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
    let mut buffer: Vec<usize> = vec![0];
    let mut position = 0;
    for i in 1..50_000_001 {
        position += steps + 1;
        position %= buffer.len();
        buffer.insert(position, i);
    }
    let index = buffer.iter().position(|&x| x == 0).unwrap();
    return buffer[(index + 1) % buffer.len()];
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        assert_eq!(638, super::part1(3));
    }
}