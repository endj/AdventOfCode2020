use std::env;
use std::fs;
use std::collections::HashSet;

fn main() {
    let file_name = env::args().skip(1).next().expect("expected input filepath argument");
    let content = fs::read_to_string(file_name).unwrap();
    let numbers: Vec<i32> = content.split_whitespace().map(|s| s.parse().unwrap()).collect();
    
    part_one(&numbers);
    part_two(&numbers);
}


fn part_one(numbers: &Vec<i32>) {
    let mut set: HashSet<i32> = HashSet::new();

    for n in numbers {
        if set.contains(n) {
            println!("{}", n * (2020-n));
            return;    
        }
        set.insert(2020-n);
    }
}


// a + b + c == 2020 -> a + b == 2020 - c
fn part_two(numbers: &Vec<i32>) {
    let mut set: HashSet<i32> = HashSet::new();

    for i in numbers {
        set.insert(2020-i);
    }

    for i in numbers {
        for j in numbers {
            if set.contains(&(i+j)) {
                print!("{}",i*j*-(i+j-2020));
                return;
            }
        }    
    }
}
