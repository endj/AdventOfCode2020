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

fn part_one(content: &str) {
    let lines: Vec<&str> = content.trim().split_whitespace().collect();
    let earliest: usize = lines[0].parse().unwrap();

    let (buss_id, actual): (usize, usize) = lines[1]
        .split(",")
        .filter(|c| *c != "x")
        .map(|x| x.parse::<usize>().unwrap())
        .map(|p| (p, (earliest / p) * p + p))
        .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
        .unwrap();
    let wait = actual - earliest;
    println!("part-one {}", buss_id * wait);
}

fn part_two(content: &str) {
    let lines: Vec<&str> = content.trim().split_whitespace().collect();
    let parts: Vec<usize> = lines[1]
        .split(",")
        .map(|c| if c == "x" { 0 } else { c.parse().unwrap() })
        .collect();

    let mut sum: usize = 1;
    let mut wait: usize = 1;
    for (id, &period) in parts.iter().enumerate() {
        if period == 0 {
            continue;
        }
        loop {
            if (sum + id) % period == 0 {
                wait *= period;
                break;
            }
            sum += wait;
        }
    }
    println!("part-two {}", sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {}
}
