use std::collections::HashMap;
use std::collections::VecDeque;
use std::env;
use std::fs;

fn main() {
    let file_name = env::args().skip(1).next().unwrap();
    let content = fs::read_to_string(file_name).unwrap();

    part_one(&content);
    part_two(&content);
}

fn part_one(content: &str) {
    let mut numbers: Vec<isize> = content
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    numbers.sort_unstable();
    let mut one = 1;
    let mut three = 1;
    for i in 0..numbers.len() - 1 {
        match numbers[i + 1] - numbers[i] {
            1 => one += 1,
            3 => three += 1,
            _ => return,
        }
    }
    println!("part-one {}", one * three);
}

#[derive(Debug)]
struct Test {
    start_index: usize,
    next: usize,
}

fn part_two(content: &str) {
    let mut numbers: Vec<usize> = content
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    numbers.push(0);

    numbers.sort_by(|a, b| b.cmp(a));

    let min = numbers.last().unwrap();
    let mut map: HashMap<usize, usize> = HashMap::new();
    println!("part-two {}", dfs(0, &numbers, &mut map, *min));
}

fn dfs(
    index: usize,
    numbers: &Vec<usize>,
    mut map: &mut HashMap<usize, usize>,
    min: usize,
) -> usize {
    if numbers[index] == min {
        return 1;
    }
    if map.contains_key(&index) {
        return *map.get(&index).unwrap();
    }
    let mut count = 0;
    for i in (index+1)..(numbers.len())  {
        if &numbers[index] - &numbers[i] <=3  {
            count += dfs(i, &numbers, &mut map, min);
        }
    }
    map.insert(index, count);
    return count;
}

