use std::usize;

pub fn solve() {
    let part1 = part1("ffayrhll");
    println!("{}", part1);
}

fn part1(input: &str) -> usize {
    let mut sum = 0;
    for index in 0..128 {
        sum += knot_hash(&format!("{}-{}", input, index)).iter().filter(|x| **x).count();
    }
    return sum;
}

fn part2(input: &str) {
    let mut bool_grid: Vec<Vec<bool>> = Vec::new();
    for index in 0..127 {
        bool_grid.push(knot_hash(&format!("{}-{}", input, index)));
    }

    let mut group = 0;
    for i in bool_grid {
        for j in i {

        }
    }
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

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        assert_eq!(8108, super::part1("flqrgnkx"));
    }
}