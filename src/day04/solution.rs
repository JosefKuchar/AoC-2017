use std::fs::File;
use std::io::prelude::*;
use std::error::Error;

pub fn solve() {
    let contents = load_input().unwrap();

    let solution1 = part1(&contents);
    let solution2 = part2(&contents);

    println!("{} {}", solution1, solution2);
}

fn load_input() -> Result<String, Box<Error>> {
    let mut file = File::open("src/day04/input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    return Ok(contents);
}

fn is_anagram(word1: &str, word2: &str) -> bool {
    for character in "abcdefghijklmnopqrstuvwxyz".chars() {
        if word1.chars().filter(|x| *x == character).count() != word2.chars().filter(|x| *x == character).count() {
            return false
        }
    }
    return true;
}

fn valid_passphrase(words: Vec<&str>) -> bool {
    for (i, word1) in words.iter().enumerate() {
        for (j, word2) in words.iter().enumerate() {
            if i != j && word1 == word2 {
                return false;
            }
        }
    }
    return true;
}

fn valid_passphrase_anagram(words: Vec<&str>) -> bool {
    for (i, word1) in words.iter().enumerate() {
        for (j, word2) in words.iter().enumerate() {
            if i != j && is_anagram(word1, word2) {
                return false;
            }
        }
    }
    return true;
}

fn part1(input: &String) -> usize {
    return input.split('\n')
        .map(|x| valid_passphrase(x.split(' ').collect()))
        .filter(|x| *x == true)
        .count();
}

fn part2(input: &String) -> usize {
    return input.split('\n')
        .map(|x| valid_passphrase_anagram(x.split(' ').collect()))
        .filter(|x| *x == true)
        .count();
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        assert_eq!(true, super::valid_passphrase("aa bb cc dd ee".split(' ').collect()));
        assert_eq!(false, super::valid_passphrase("aa bb cc dd aa".split(' ').collect()));
        assert_eq!(true, super::valid_passphrase("aa bb cc dd aaa".split(' ').collect()));
    }

    #[test]
    fn part2() {
        assert_eq!(true, super::valid_passphrase_anagram("abcde fghij".split(' ').collect()));
        assert_eq!(false, super::valid_passphrase_anagram("abcde xyz ecdab".split(' ').collect()));
        assert_eq!(true, super::valid_passphrase_anagram("a ab abc abd abf abj".split(' ').collect()));
        assert_eq!(true, super::valid_passphrase_anagram("iiii oiii ooii oooi oooo".split(' ').collect()));
        assert_eq!(false, super::valid_passphrase_anagram("oiii ioii iioi iiio".split(' ').collect()));
    }
}