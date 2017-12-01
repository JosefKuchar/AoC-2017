use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::open("input.txt").expect("not found");
    let mut text = String::new();
    let mut sum: usize = 0;
    file.read_to_string(&mut text).expect("error");
    
    for (index, character) in text.chars().enumerate() {
        if character == text.chars().nth((index + 1) % text.len()).unwrap() {
            sum += character.to_digit(10).unwrap() as usize;
        }
    }

    println!("{}", sum);
}
