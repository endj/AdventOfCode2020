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
        let id = seat_id(calculate_seat_part_one(&board));
        if id > max {
            max = id;
        }
    }
    println!("part-one {}", max);
}

fn part_two(content: &String) {
    let boardings = content.split_whitespace();
    let mut seats = Vec::new();

    for board in boardings {
        let id = seat_id(calculate_seat_part_one(&board));
        seats.push(id);
    }
    seats.sort();

    for (i, id) in seats.iter().enumerate() {
        if id != &(&seats[i + 1] - 1) {
            println!("part-two {}", id + 1);
            break;
        }
    }
}

fn calculate_seat_part_one(board: &str) -> (i32, i32) {
    let mut r_l = 0;
    let mut r_r = 127;
    let mut s_l = 0;
    let mut s_r = 7;

    for c in board.chars() {
        match c {
            'F' => r_r = mid(r_l, r_r, 'F'),
            'B' => r_l = mid(r_l, r_r, 'B'),
            'L' => s_r = mid(s_l, s_r, 'L'),
            'R' => s_l = mid(s_l, s_r, 'R'),
            _ => (),
        }
    }

    return (r_l, s_l);
}

fn seat_id((row, column): (i32, i32)) -> i32 {
    row * 8 + column
}

fn mid(left: i32, right: i32, sign: char) -> i32 {
    return match sign {
        'F' | 'L' => (left + right) / 2,
        'B' | 'R' => ((left + right) / 2) + 1,
        _ => 1,
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
        assert_eq!(calculate_seat_part_one("FBFBBFFRLR"), (44, 5));
        assert_eq!(calculate_seat_part_one("BFFFBBFRRR"), (70, 7));
        assert_eq!(calculate_seat_part_one("FFFBBBFRRR"), (14, 7));
        assert_eq!(calculate_seat_part_one("BBFFBBFRLL"), (102, 4));
    }

    #[test]
    fn test_mid() {
        assert_eq!(mid(0, 127, 'F'), 63);
        assert_eq!(mid(0, 63, 'B'), 32);
        assert_eq!(mid(32, 63, 'F'), 47);
        assert_eq!(mid(32, 47, 'B'), 40);
        assert_eq!(mid(40, 47, 'B'), 44);
        assert_eq!(mid(44, 47, 'F'), 45);
        assert_eq!(mid(44, 45, 'F'), 44);

        assert_eq!(mid(0, 7, 'R'), 4);
        assert_eq!(mid(4, 7, 'L'), 5);
        assert_eq!(mid(4, 5, 'R'), 5);
    }
}
