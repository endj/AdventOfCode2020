use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs;

fn main() {
    let file_name = env::args()
        .skip(1)
        .next()
        .expect("expected input filepath argument");
    let content = fs::read_to_string(file_name).unwrap();

    part_one(&content);
    part_two(&content);
}

fn part_one(content: &String) {
    let lines = content.split("\n\n");
    let mut options = HashSet::new();
    let mut sum = 0;

    for group in lines {
        for c in group.chars() {
            if c != '\n' {
                options.insert(c);
            }
        }
        sum += options.len();
        options.clear();
    }
    println!("part-one {}", sum);
}

fn part_two(content: &String) {
    let lines = content.split("\n\n");
    let mut options: HashMap<char, u32> = HashMap::new();
    let mut sum = 0;

    for group in lines {
        let mut group_size = 1;
        for c in group.trim().chars() {
            if c != '\n' {
                *options.entry(c).or_insert(0) += 1;
            } else {
                group_size += 1;
            }
        }
        for v in options.values() {
            if v == &group_size {
                sum += 1;
            }
        }
        options.clear();
    }
    println!("part-two {}", sum);
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {}
}
