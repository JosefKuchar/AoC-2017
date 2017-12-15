pub fn solve() {
    let part1 = part1(699, 124);
    let part2 = part2(699, 124);
    println!("{} {}", part1, part2);
}

fn part1(a: usize, b: usize) -> usize {
    let mut a = a;
    let mut b = b;
    let mut matches = 0;
    for _ in 0..40_000_000 {
        a = (a * 16807) % 2147483647;
        b = (b * 48271) % 2147483647;
        if a & 0xffff == b & 0xffff {
            matches += 1;
        }
    }
    
    return matches;
}

fn part2(a: usize, b: usize) -> usize {
    let mut a = a;
    let mut b = b;
    let mut matches = 0;
    for _ in 0..5_000_000 {
        loop {
            a = (a * 16807) % 2147483647;
            if a % 4 == 0 {
                break;
            }
        }

        loop {
            b = (b * 48271) % 2147483647;
            if b % 8 == 0 {
                break;
            }
        }

        if a & 0xffff == b & 0xffff {
            matches += 1;
        }
    }
    return matches;
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        assert_eq!(588, super::part1(65, 8921));
    }

    #[test]
    fn part2() {
        assert_eq!(309, super::part2(65, 8921));
    }
}