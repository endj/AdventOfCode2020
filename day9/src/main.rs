use std::cmp;
use std::collections::HashSet;
use std::env;
use std::fs;

fn main() {
    let file_name = env::args().skip(1).next().unwrap();
    let content = fs::read_to_string(file_name).unwrap();

    let invalid_number = part_one(&content);
    part_two(&content, invalid_number);
}

// foreach n create pair lookuptable previous 25
// if no pair return
fn part_one(content: &str) -> isize {
    let numbers: Vec<isize> = content
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let preamble: usize = 25;

    let mut set: HashSet<isize> = HashSet::new();
    for i in preamble..numbers.len() {
        let start_index = cmp::max(i - preamble, 0);

        let target = numbers[i];
        let mut has_pair = false;

        for j in start_index..i as usize {
            let num = numbers[j as usize];
            let pair = target - num;
            if set.contains(&num) {
                has_pair = true;
                break;
            }
            set.insert(pair as isize);
        }
        if !has_pair {
            println!("part-one {}", numbers[i]);
            return numbers[i];
        }
        set.clear();
    }
    return -1;
}
// find a sequence that adds up to part-one number
// sliding window until bigger then left until fit update on slide
fn part_two(content: &str, invalid_number: isize) {
    let numbers: Vec<isize> = content
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut left = 0;
    let mut right = 1;
    let mut sum = &numbers[0] + &numbers[1];

    while sum != invalid_number {
        if sum < invalid_number {
            right += 1;
            sum += &numbers[right];
        } else if sum > invalid_number {
            sum -= &numbers[left];
            left += 1;
            if left == right {
                right += 1;
                sum += &numbers[right];
            }
        }
    }
    let mut min = numbers[left] as usize;
    let mut max = numbers[right] as usize;
    for i in left..right {
        let val = numbers[i] as usize;
        if val > max {
            max = val;
        }
        if val < min {
            min = val;
        }
    }

    println!("part-two {}", min + max);
}
