//use std::collections::HashMap;
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

}


fn part_two(content: &str) {

}

#[cfg(test)]
mod tests {
    use super::*;
}
