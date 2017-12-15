pub fn solve() {
    let part1 = part1(699, 124);
    println!("{}", part1);
}

fn part1(a: usize, b: usize) -> usize {
    let mut a = a;
    let mut b = b;
    let mut matches = 0;
    for _ in 0..40_000_000 {
        a = (a * 16807) % 2147483647;
        b = (b * 48271) % 2147483647;
        /*
        if &format!("{:032b}", a)[16..] == &format!("{:032b}", b)[16..] {
            matches += 1;
        }*/
        if a & 0xffff == b & 0xffff {
            matches += 1;
        }
    }
    
    return matches;
}

fn part2(a: usize, b: usize) -> usize {
    let mut a = a;
    let mut b = b;
    for _ in 0..5_000_000 {

    }
    return 0;
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        assert_eq!(588, super::part1(65, 8921));
    }
}