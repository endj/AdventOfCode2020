use std::env;
use std::fs;

fn main() {
    let file_name = env::args().skip(1).next().expect("expected input filepath argument");
    let content = fs::read_to_string(file_name).unwrap();

    part_one(&content);
    part_two(&content);
}

fn traverse(dx: i32, dy: i32, content: &String) -> i32 {
    let lines = content.lines();
    let mut x: i32 = 0;
    let mut treehits: i32 = 0;
    for line in lines.step_by(dy as usize) {
        let width = line.len();
        match line.chars().nth((x as usize) % width) {
            Some('#') => treehits += 1,
            _ => ()
        };
        x += dx;
    }
    return treehits;
}

    
fn part_two(content: &String) {
    let variations = vec![(1,1),(3,1),(5,1),(7,1),(1,2)];
    let mut sum: u64  = 1;
    for pair in variations {
        let result = traverse(pair.0, pair.1, &content);
        sum *= result as u64;
    }
    println!("part-two {}", sum);
}

fn part_one(content: &String) {
    let result = traverse(3, 1, &content);
    println!("part-one {} ", result);
}
