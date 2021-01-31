use std::collections::HashSet;
use std::env;
use std::fs;

#[derive(Hash, Debug, Eq, PartialEq, Clone)]
struct Cube {
    x: isize,
    y: isize,
    z: isize,
    on: bool,
}

#[derive(Hash, Debug, Eq, PartialEq, Clone)]
struct Cube2 {
    x: isize,
    y: isize,
    z: isize,
    w: isize,
    on: bool,
}

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
    let mut cubes = parse_cubes3d(content);
    for _ in 0..6 {
        cubes = next_state3d(&mut cubes);
    }

    let x = cubes.iter().filter(|c| c.on).count();
    println!("part-one {}", x);
}

fn part_two(content: &str) {
    let mut cubes = parse_cubes4d(content);

    for _ in 0..6 {
        cubes = next_state4d(&mut cubes);
    }

    let x = cubes.iter().filter(|c| c.on).count();
    println!("part-two {}", x);
}

fn next_state3d(mut cubes: &mut HashSet<Cube>) -> HashSet<Cube> {
    add_nearby_cubes3d(&mut cubes);
    let mut new = HashSet::new();

    for c in cubes.iter() {
        let active_neighbours = neighbours3d(&c, &cubes);

        match c.on {
            true => {
                if active_neighbours == 2 || active_neighbours == 3 {
                    new.insert(Cube {
                        x: c.x,
                        y: c.y,
                        z: c.z,
                        on: true,
                    });
                } else {
                    new.insert(Cube {
                        x: c.x,
                        y: c.y,
                        z: c.z,
                        on: false,
                    });
                }
            }
            false => {
                if active_neighbours == 3 {
                    new.insert(Cube {
                        x: c.x,
                        y: c.y,
                        z: c.z,
                        on: true,
                    });
                } else {
                    new.insert(Cube {
                        x: c.x,
                        y: c.y,
                        z: c.z,
                        on: false,
                    });
                }
            }
        }
    }
    new
}

fn next_state4d(mut cubes: &mut HashSet<Cube2>) -> HashSet<Cube2> {
    add_nearby_cubes4d(&mut cubes);
    let mut new = HashSet::new();

    for c in cubes.iter() {
        let active_neighbours = neighbours4d(&c, &cubes);

        match c.on {
            true => {
                if active_neighbours == 2 || active_neighbours == 3 {
                    new.insert(Cube2 {
                        x: c.x,
                        y: c.y,
                        z: c.z,
                        w: c.w,
                        on: true,
                    });
                } else {
                    new.insert(Cube2 {
                        x: c.x,
                        y: c.y,
                        z: c.z,
                        w: c.w,
                        on: false,
                    });
                }
            }
            false => {
                if active_neighbours == 3 {
                    new.insert(Cube2 {
                        x: c.x,
                        y: c.y,
                        z: c.z,
                        w: c.w,
                        on: true,
                    });
                } else {
                    new.insert(Cube2 {
                        x: c.x,
                        y: c.y,
                        z: c.z,
                        w: c.w,
                        on: false,
                    });
                }
            }
        }
    }
    new
}

fn add_nearby_cubes3d(cubes: &mut HashSet<Cube>) {
    let cube_clone = cubes.clone();

    let cubes_to_add = cube_clone
        .iter()
        .filter(|c| c.on)
        .flat_map(|c| missing_cubes_around_cube3d(&c, &cube_clone));

    for c in cubes_to_add {
        &cubes.insert(c);
    }
}

fn add_nearby_cubes4d(cubes: &mut HashSet<Cube2>) {
    let cube_clone = cubes.clone();

    let cubes_to_add = cube_clone
        .iter()
        .filter(|c| c.on)
        .flat_map(|c| missing_cubes_around_cube4d(&c, &cube_clone));

    for c in cubes_to_add {
        &cubes.insert(c);
    }
}

fn missing_cubes_around_cube3d(cube: &Cube, neighbours: &HashSet<Cube>) -> HashSet<Cube> {
    let mut new_cubes = HashSet::new();

    for x in cube.x - 1..cube.x + 2 {
        for y in cube.y - 1..cube.y + 2 {
            for z in cube.z - 1..cube.z + 2 {
                if cube.x == x && cube.y == y && cube.z == z {
                    continue;
                }
                match neighbours.get(&Cube {
                    x: x,
                    y: y,
                    z: z,
                    on: true,
                }) {
                    Some(res) => {}
                    None => {
                        new_cubes.insert(Cube {
                            x: x,
                            y: y,
                            z: z,
                            on: false,
                        });
                    }
                };
            }
        }
    }
    new_cubes
}

fn missing_cubes_around_cube4d(cube: &Cube2, neighbours: &HashSet<Cube2>) -> HashSet<Cube2> {
    let mut new_cubes = HashSet::new();
    for w in cube.w - 1..cube.w + 2 {
        for x in cube.x - 1..cube.x + 2 {
            for y in cube.y - 1..cube.y + 2 {
                for z in cube.z - 1..cube.z + 2 {
                    if cube.x == x && cube.y == y && cube.z == z && cube.w == w {
                        continue;
                    }
                    match neighbours.get(&Cube2 {
                        x: x,
                        y: y,
                        z: z,
                        w: w,
                        on: true,
                    }) {
                        Some(res) => {}
                        None => {
                            new_cubes.insert(Cube2 {
                                x: x,
                                y: y,
                                z: z,
                                w: w,
                                on: false,
                            });
                        }
                    };
                }
            }
        }
    }
    new_cubes
}

fn neighbours3d(cube: &Cube, neighbours: &HashSet<Cube>) -> usize {
    let mut count = 0;
    for z in cube.z - 1..cube.z + 2 {
        for y in cube.y - 1..cube.y + 2 {
            for x in cube.x - 1..cube.x + 2 {
                if cube.x == x && cube.y == y && cube.z == z {
                    continue;
                }
                match neighbours.get(&Cube {
                    x: x,
                    y: y,
                    z: z,
                    on: true,
                }) {
                    Some(_) => count += 1,
                    None => (),
                }
            }
        }
    }
    count
}

fn neighbours4d(cube: &Cube2, neighbours: &HashSet<Cube2>) -> usize {
    let mut count = 0;
    for z in cube.z - 1..cube.z + 2 {
        for w in cube.w - 1..cube.w + 2 {
            for y in cube.y - 1..cube.y + 2 {
                for x in cube.x - 1..cube.x + 2 {
                    if cube.x == x && cube.y == y && cube.z == z && cube.w == w {
                        continue;
                    }
                    match neighbours.get(&Cube2 {
                        x: x,
                        y: y,
                        z: z,
                        w: w,
                        on: true,
                    }) {
                        Some(_) => count += 1,
                        None => (),
                    }
                }
            }
        }
    }
    count
}

fn parse_cubes4d(content: &str) -> HashSet<Cube2> {
    let mut x = 0;
    let mut y = 0;
    let mut cubes = HashSet::new();
    for line in content.lines() {
        for state in line.chars() {
            cubes.insert(Cube2 {
                x: x,
                y: y,
                z: 0,
                w: 0,
                on: match state {
                    '#' => true,
                    _ => false,
                },
            });
            x += 1
        }
        y -= 1;
        x = 0;
    }
    cubes
}

fn parse_cubes3d(content: &str) -> HashSet<Cube> {
    let mut x = 0;
    let mut y = 0;
    let mut cubes = HashSet::new();
    for line in content.lines() {
        for state in line.chars() {
            cubes.insert(Cube {
                x: x,
                y: y,
                z: 0,
                on: match state {
                    '#' => true,
                    _ => false,
                },
            });
            x += 1
        }
        y -= 1;
        x = 0;
    }
    cubes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn neigh() {
        let mut map: HashSet<Cube> = HashSet::new();
        map.insert(Cube {
            x: 0,
            y: -1,
            z: 0,
            on: true,
        });
        map.insert(Cube {
            x: 1,
            y: -1,
            z: 0,
            on: true,
        });
        map.insert(Cube {
            x: 2,
            y: -1,
            z: 0,
            on: true,
        });
        let n = neighbours(
            &Cube {
                x: 1,
                y: 0,
                z: 0,
                on: false,
            },
            &map,
        );
        assert_eq!(n, 3);
    }
}
