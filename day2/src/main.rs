use std::env;
use std::fs;

fn main() {
    let file_name = env::args().skip(1).next().expect("expected input filepath argument");
    let content = fs::read_to_string(file_name).unwrap();

    part_one(&content);
    part_two(&content);
}


fn part_one(content: &String) {
    let lines = content.lines();

    let mut number_of_valid_passwords = 0;
    for line in lines {
        let mut parts = line.trim().split_whitespace();
        let occurence = parts.next().unwrap();
        let character = parts.next().unwrap().chars().next().unwrap();
        let password = parts.next().unwrap(); 
        
        let mut parts = occurence.split("-");
        let min = parts.next().unwrap().parse().unwrap();   
        let max = parts.next().unwrap().parse().unwrap();   

        if line.len() < min {
            continue;
        }
        
        let mut occurences = 0;
        for c in password.chars() {
            if c == character {
                occurences += 1;
            }
        }
        if occurences > max || occurences < min {
            continue;
        }
        number_of_valid_passwords += 1;
    }
    println!("Number of valid passwords part-one {}", number_of_valid_passwords);
}


fn part_two(content: &String) {
    let lines = content.lines();

    let mut number_of_valid_passwords = 0;
    for line in lines {
        let mut parts = line.trim().split_whitespace();
        let occurence = parts.next().unwrap();
        let character = parts.next().unwrap().chars().next().unwrap();
        let password = parts.next().unwrap();

        let mut parts = occurence.split("-");
        let left_position: i32 = parts.next().unwrap().parse().unwrap();
        let right_position: i32 = parts.next().unwrap().parse().unwrap();

        let first = match password.chars().nth((left_position - 1) as usize) {
            None => false,
            Some(c) => c == character
        };
        let second = match password.chars().nth((right_position - 1) as usize) {
            None => false,
            Some(c) => c == character
        };

        if first ^ second {
            number_of_valid_passwords += 1;
        }
    }
    println!("Number of valid passwords part-two {}", number_of_valid_passwords);
}   


