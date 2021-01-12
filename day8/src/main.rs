use std::collections::HashSet;
use std::env;
use std::fs;

#[derive(Debug)]
struct Halts {
    halts: bool,
    accumulator: isize,
}

fn main() {
    let file_name = env::args()
        .skip(1)
        .next()
        .expect("expected input filepath argument");
    let content = fs::read_to_string(file_name).unwrap();

    let lines = content.trim().split("\n");
    let mut instructions: Vec<(&str, isize, char)> = Vec::new();

    for line in lines {
        let mut parts = line.split_whitespace();
        let instruction = &parts.next().unwrap();
        let value_sign = &parts.next().unwrap();
        let sign = value_sign.chars().next().unwrap();
        let value = match sign {
            '+' => parse_value_sign("+", value_sign),
            '-' => parse_value_sign("-", value_sign),
            _ => return,
        };
        instructions.push((instruction, value, sign));
    }

    part_one(&instructions);
    part_two(&instructions);
}

fn parse_value_sign(sign: &str, val: &str) -> isize {
    val.split(sign).skip(1).next().unwrap().parse().unwrap()
}

fn halts(instructions: &Vec<(&str, isize, char)>) -> Halts {
    let mut seen: HashSet<isize> = HashSet::new();
    let mut ip: isize = 0;
    let mut acc: isize = 0;

    loop {
        if ip >= instructions.len() as isize {
            return Halts {
                halts: true,
                accumulator: acc,
            };
        }
        let next = instructions[ip as usize];
        let ins: &str = next.0;
        let value: isize = next.1;
        let sign: char = next.2;

        if seen.contains(&ip) {
            return Halts {
                halts: false,
                accumulator: acc,
            };
        }
        seen.insert(ip);

        match ins {
            "jmp" => ip += if sign == '+' { value } else { -(value) },
            "acc" => {
                acc += if sign == '+' { value } else { -(value) };
                ip += 1;
            }
            "nop" => ip += 1,
            _ => (),
        };
    }
}

// test every jmp if halts
fn part_two(instructions: &Vec<(&str, isize, char)>) {
    let mut index = 0;
    for ins in instructions {
        let mut instruct = instructions.clone();
        if ins.0 == "jmp" {
            let old = instruct[index];
            instruct[index] = ("nop", old.1, old.2);
            let res = halts(&instruct);
            if res.halts {
                println!("part-two {}", res.accumulator);
                return;
            }
        }
        index += 1;
    }
}

fn part_one(instructions: &Vec<(&str, isize, char)>) {
    let halted = halts(&instructions);
    println!("part-one {}", halted.accumulator);
}
