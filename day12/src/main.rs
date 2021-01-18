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

    let mut x: isize = 0;
    let mut y: isize = 0;
    let mut dx: isize = 1;
    let mut dy: isize = 0;

    for line in lines {
        let direction = &line[0..1];
        let val: isize = line[1..line.len()].parse().unwrap();

        match direction {
            "N" => y += val,
            "S" => y -= val,
            "E" => x += val,
            "W" => x -= val,
            "F" => {
                x += dx * val;
                y += dy * val;
            }
            "R" => {
                let (ndx, ndy) = rotate(dx, dy, val);
                dx = ndx;
                dy = ndy;
            }
            "L" => {
                let (ndx, ndy) = rotate(dx, dy, -val);
                dx = ndx;
                dy = ndy;
            }
            _ => (),
        }
    }
    println!("part-one {}", x.abs() + y.abs());
}

fn rotate(dx: isize, dy: isize, rotation_value: isize) -> (isize, isize) {
    let rotations = rotation_value / 90;
    let directions = [
        (1, 0),  // east
        (0, -1), // south
        (-1, 0), // west
        (0, 1),  // north
    ];
    let index = directions
        .iter()
        .position(|dir| dx == dir.0 && dy == dir.1)
        .unwrap() as isize;

    let new_position = (index + rotations).rem_euclid(directions.len() as isize) as usize;
    directions[new_position]
}

fn part_two(content: &str) {
    let lines: Vec<&str> = content.trim().split_whitespace().collect();

    let mut wx: isize = 10;
    let mut wy: isize = 1;

    let mut x: isize = 0;
    let mut y: isize = 0;

    for line in lines {
        let direction = &line[0..1];
        let val: isize = line[1..line.len()].parse().unwrap();

        match direction {
            "N" => wy += val,
            "S" => wy -= val,
            "E" => wx += val,
            "W" => wx -= val,
            "F" => {
                x += wx * val;
                y += wy * val;
            }
            "R" => {
                let (nwx, nwy) = rotate_waypoint(wx, wy, val);
                wx = nwx;
                wy = nwy;
            }
            "L" => {
                let (nwx, nwy) = rotate_waypoint(wx, wy, -val);
                wx = nwx;
                wy = nwy;
            }
            _ => (),
        }
    }
    println!("part-two {}", x.abs() + y.abs()); 
}

fn rotate_waypoint(wx: isize, wy: isize, rotation_value: isize) -> (isize, isize) {
    let mut nwx = wx;
    let mut nwy = wy;
    let rotations = (rotation_value / 90).abs();
    for _ in 0..rotations {
        if rotation_value > 0 {
            let temp = -nwx;
            nwx = nwy;
            nwy = temp;
        } else {
            let temp = -nwy;
            nwy = nwx;
            nwx = temp;
        }
    }
    (nwx, nwy)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rotate_waypoint_right() {
        assert_eq!((4, -10), rotate_waypoint(10, 4, 90));
    }

    #[test]
    fn rotate_waypoint_left() {
        assert_eq!((10, 4), rotate_waypoint(4, -10, -90));
    }

    #[test]
    fn full_rotation_right() {
        let directions = [
            (1, 0),  // east
            (0, -1), // south
            (-1, 0), // west
            (0, 1),  // north
        ];

        for dir in directions.iter() {
            let new_dir = rotate(dir.0, dir.1, 360);
            assert_eq!(dir.0, new_dir.0);
            assert_eq!(dir.1, new_dir.1);
        }
    }

    #[test]
    fn rot_left() {
        let dir = (-1, 0); // west
        let new_dir = rotate(dir.0, dir.1, -90);
        assert_eq!(0, new_dir.0);
        assert_eq!(-1, new_dir.1);
    }

    #[test]
    fn rot_right() {
        let dir = (-1, 0); // west
        let new_dir = rotate(dir.0, dir.1, 90);
        assert_eq!(0, new_dir.0);
        assert_eq!(1, new_dir.1);
    }
}
