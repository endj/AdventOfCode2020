use std::collections::HashMap;
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
    let last = play_game(words(content), 2020);
    println!("part-one {}", last);
}

fn play_game(mut words: Vec<usize>, turns: usize) -> usize {
    let mut word_map: HashMap<usize, usize> = HashMap::new();
    let mut turn = 1;
    let mut last = words.pop().unwrap();
   
    for w in &words {
        word_map.insert(*w, turn);
        turn += 1;
    }

    while turn != turns {
        last = eval_next(&mut word_map, last, turn);
        turn += 1;
    }
    last
}

fn words(content: &str) -> Vec<usize> {
    content.trim().split(",").map(|w| w.parse().unwrap()).collect()
}

fn eval_next(word_map: &mut HashMap<usize, usize>, last: usize, turn: usize) -> usize {
    let new = match word_map.get(&last) {
        Some(n) => turn - n,
        None => 0,
    };
    word_map.insert(last, turn);
    new
}

fn part_two(content: &str) {
  println!("part-two {}", play_game(words(content), 30000000));
}
