use std::env;
use std::fs;


// day 11 Copy and paste edition
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
    let mut seats: Vec<Vec<char>> = content
        .trim()
        .split_whitespace()
        .map(|l| l.chars().collect())
        .collect();

    loop {
        //        print_seats(&seats);
        let (new_state, seats_changed) = advance(&seats);
        if seats_changed == 0 {
            break;
        }
        seats = new_state;
    }
    println!("part-one {}", count_seated(&seats));
}

fn count_seated(seats: &Vec<Vec<char>>) -> usize {
    let mut seated = 0;
    for row in seats {
        for col in row {
            if col == &'#' {
                seated += 1;
            }
        }
    }
    return seated;
}

fn advance(read: &Vec<Vec<char>>) -> (Vec<Vec<char>>, usize) {
    let mut cloned: Vec<Vec<char>> = read.clone();
    let mut seats_changed = 0;
    for y in 0..read.len() {
        for x in 0..read[y].len() {
            let seat = &read[y][x];
            if seat == &'.' {
                continue;
            }

            let adj = adjecent(y, x, &read, '#');
            if seat == &'L' && adj == 0 {
                cloned[y][x] = '#';
                seats_changed += 1;
            }
            if seat == &'#' && adj >= 4 {
                cloned[y][x] = 'L';
                seats_changed += 1;
            }
        }
    }
    return (cloned, seats_changed);
}

fn print_seats(seats: &Vec<Vec<char>>) {
    for r in seats {
        println!();
        for c in r {
            print!("{}", c);
        }
    }
    println!();
}

//     return 0 <= x < len(grid[0]) and 0 <= y < len(grid)
fn in_bounds(y: isize, x: isize, seats: &Vec<Vec<char>>) -> bool {
    0 <= y && (y as usize) < seats.len() && 0 <= x && (x as usize) < seats[0].len()
}

fn adjecent(y: usize, x: usize, seats: &Vec<Vec<char>>, check: char) -> usize {
    let mut adjecent = 0;
    let directions: [(isize, isize); 8] = [
        (-1, 1),
        (0, 1),
        (1, 1),
        (-1, 0),
        (1, 0),
        (-1, -1),
        (0, -1),
        (1, -1),
    ];

    for &(dx, dy) in directions.iter() {
        let check_y = y as isize + dy;
        let check_x = x as isize + dx;
        if in_bounds(check_y as isize, check_x as isize, &seats) {
            if seats[check_y as usize][check_x as usize] == check {
                adjecent += 1;
            }
        }
    }
    return adjecent;
}

fn in_view(y: usize, x: usize, seats: &Vec<Vec<char>>, check: char) -> usize {
    let mut adjecent = 0;
    let directions: [(isize, isize); 8] = [
        (-1, 1),
        (0, 1),
        (1, 1),  
        (-1, 0),
        (1, 0),
        (-1, -1),
        (0, -1),
        (1, -1),
    ];

    for &(dx, dy) in directions.iter() {
        let mut check_y = y as isize + dy;
        let mut check_x = x as isize + dx;

        while in_bounds(check_y as isize, check_x as isize, &seats) && seats[check_y as usize][check_x as usize] == '.' {
            check_y = check_y + dy;
            check_x = check_x + dx;
        }
        if in_bounds(check_y as isize, check_x as isize, &seats) {
            if seats[check_y as usize][check_x as usize] == check {
                adjecent += 1;
            }
        }
    }
    return adjecent;
}

fn advance_two(read: &Vec<Vec<char>>) -> (Vec<Vec<char>>, usize) {
    let mut cloned: Vec<Vec<char>> = read.clone();
    let mut seats_changed = 0;
    for y in 0..read.len() {
        for x in 0..read[y].len() {
            let seat = &read[y][x];
            if seat == &'.' {
                continue;
            }

            let adj = in_view(y, x, &read, '#');
            if seat == &'L' && adj == 0 {
                cloned[y][x] = '#';
                seats_changed += 1;
            }
            if seat == &'#' && adj >= 5 {
                cloned[y][x] = 'L';
                seats_changed += 1;
            }
        }
    }
    return (cloned, seats_changed);
}

fn part_two(content: &str) {
        let mut seats: Vec<Vec<char>> = content
        .trim()
        .split_whitespace()
        .map(|l| l.chars().collect())
        .collect();

    loop {
        //        print_seats(&seats);
        let (new_state, seats_changed) = advance_two(&seats);
        if seats_changed == 0 {
            break;
        }
        seats = new_state;
    }
    println!("part-two {}", count_seated(&seats));
}
