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
    let boardings = content.split_whitespace();
    let mut max = 0;
    for board in boardings {
        let id = seat_id(&board);
        if id > max {
            max = id;
        }
    }
    println!("part-one {}", max);
}

fn part_two(content: &String) {
    let boardings = content.split_whitespace();

    let mut found_seat_sum = 0;
    let mut max = 0;
    let mut seats = 0;

    for board in boardings {
        let id = seat_id(&board);
        found_seat_sum += id;
        if id > max {
            max = id;
        }
        seats += 1;
    }

    println!(
        "part-two {}",
        sum_of_series_range(max - seats, max) - found_seat_sum
    );
}

fn sum_of_series_range(from: i128, to: i128) -> i128 {
    ((to - from + 1) * (from + to)) / 2
}

fn seat_id(board: &str) -> i128 {
    let mut id = 0;
    for (i, b) in board.as_bytes().iter().rev().enumerate() {
        if b == &b'B' || b == &b'R' {
            id |= 1 << i;
        }
    }
    return id;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_seat_id() {
        assert_eq!(seat_id("FBFBBFFRLR"), 357);
        assert_eq!(seat_id("BFFFBBFRRR"), 567);
        assert_eq!(seat_id("FFFBBBFRRR"), 119);
    }
}
