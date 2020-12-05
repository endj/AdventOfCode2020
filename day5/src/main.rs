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
        let id = seat_id(calculate_seat(&board));
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
        let id = seat_id(calculate_seat(&board));
        found_seat_sum += id;
        if id > max {
            max = id;
        }
        seats += 1;
    }

    let expected_seat_sum = sum_of_series_range(max - seats, max);
    println!("part-two {}", expected_seat_sum - found_seat_sum);
}

fn sum_of_series_range(from: i32, to: i32) -> i32 {
    ((to - from + 1) * (from + to)) / 2
}

fn calculate_seat(board: &str) -> (i32, i32) {
    let mut r_l = 0;
    let mut r_r = 127;
    let mut s_l = 0;
    let mut s_r = 7;

    for c in board.chars() {
        match c {
            'F' => r_r = mid(r_l, r_r, false),
            'B' => r_l = mid(r_l, r_r, true),
            'L' => s_r = mid(s_l, s_r, false),
            'R' => s_l = mid(s_l, s_r, true),
            _ => (),
        }
    }

    return (r_l, s_l);
}

fn seat_id((row, column): (i32, i32)) -> i32 {
    row * 8 + column
}

fn mid(left: i32, right: i32, add_one: bool) -> i32 {
    return match add_one {
        false => (left + right) / 2,
        true => ((left + right) / 2) + 1,
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_seat_id() {
        assert_eq!(seat_id((44, 5)), 357);
        assert_eq!(seat_id((70, 7)), 567);
        assert_eq!(seat_id((14, 7)), 119);
        assert_eq!(seat_id((102, 4)), 820);
    }

    #[test]
    fn test_calc_seats() {
        assert_eq!(calculate_seat("FBFBBFFRLR"), (44, 5));
        assert_eq!(calculate_seat("BFFFBBFRRR"), (70, 7));
        assert_eq!(calculate_seat("FFFBBBFRRR"), (14, 7));
        assert_eq!(calculate_seat("BBFFBBFRLL"), (102, 4));
    }

    #[test]
    fn test_mid() {
        assert_eq!(mid(0, 127, false), 63);
        assert_eq!(mid(0, 63, true), 32);
        assert_eq!(mid(32, 63, false), 47);
        assert_eq!(mid(32, 47, true), 40);
        assert_eq!(mid(40, 47, true), 44);
        assert_eq!(mid(44, 47, false), 45);
        assert_eq!(mid(44, 45, false), 44);

        assert_eq!(mid(0, 7, true), 4);
        assert_eq!(mid(4, 7, false), 5);
        assert_eq!(mid(4, 5, true), 5);
    }
}
