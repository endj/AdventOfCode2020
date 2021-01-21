use std::collections::HashMap;
use std::env;
use std::fs;

#[derive(Debug)]
struct Mask {
    or_mask: usize,
    and_mask: usize,
}

fn main() {
    let file_name = env::args()
        .skip(1)
        .next()
        .expect("expected input filepath argument");
    let content = fs::read_to_string(file_name)
        .unwrap()
        .replace("=", "")
        .replace("[", " ")
        .replace("]", " ");

    part_one(&content);
    part_two(&content);
}

fn part_one(content: &str) {
    let lines: Vec<&str> = content.trim().split("\n").collect();
    let mut masked_values: HashMap<usize, usize> = HashMap::new();

    let mut mask: Mask = Mask {
        and_mask: 0,
        or_mask: 0,
    };
    for l in lines {
        if l.starts_with("ma") {
            mask = parse_masks(l);
        } else {
            let (address, value) = parse_address(l);
            let masked_val = apply_mask(value, &mask);
            masked_values.insert(address, masked_val);
        }
    }
    let sum: usize = masked_values.values().sum();
    println!("part-one {}", sum);
}

fn part_two(content: &str) {
    let lines: Vec<&str> = content.trim().split("\n").collect();
    let mut masked_values: HashMap<usize, usize> = HashMap::new();

    let mut mask: &str = "";
    for i in lines {
        if i.starts_with("ma") {
            mask = i.split_whitespace().skip(1).next().unwrap();
        } else {
            let (address, value) = parse_address(i);

            let float_and_mask: usize = usize::from_str_radix(
                &*mask
                    .chars()
                    .map(|c| if c == 'X' { '0' } else { '1' })
                    .collect::<String>(),
                2,
            )
            .unwrap();
            let or_mask: usize = usize::from_str_radix(
                &*mask
                    .chars()
                    .map(|c| if c == '1' { '1' } else { '0' })
                    .collect::<String>(),
                2,
            )
            .unwrap();

            let masks: Mask = Mask {
                and_mask: float_and_mask,
                or_mask: or_mask,
            };

            let masked_address = apply_mask(address, &masks);

            let x_and_index: Vec<usize> = get_x_indexis(&mask);
            for i in 0..2_u32.pow(x_and_index.len() as u32) {
                let or_mask = or_mask_pt_two(i, &x_and_index);
                let ored_value = masked_address | or_mask;

                masked_values.insert(ored_value, value);
            }
        }
    }
    let sum: usize = masked_values.values().sum();

    println!("part-two {}", sum);
}

fn get_x_indexis(mask: &str) -> Vec<usize> {
    let mut x_and_index: Vec<usize> = Vec::new();
    for (index, c) in mask.chars().enumerate() {
        if c == 'X' {
            &x_and_index.push(35 - index);
        }
    }
    x_and_index
}

fn or_mask_pt_two(value: u32, indexis: &Vec<usize>) -> usize {
    let binary_val = format!("{:b}", value);

    let mut or_mask: usize = 0;
    let mut binary_parts = binary_val.chars().rev();

    for x_index in indexis.iter().rev() {
        let next_bit = match binary_parts.next() {
            Some(c) => c,
            None => '0',
        }
        .to_digit(10)
        .unwrap();

        or_mask = or_mask | (next_bit as usize) << x_index;
    }
    or_mask
}

fn apply_mask(value: usize, mask: &Mask) -> usize {
    (value | mask.or_mask) & mask.and_mask
}

fn parse_address(address: &str) -> (usize, usize) {
    let mut it = &mut address.split_whitespace();
    let index = &it.skip(1).next().unwrap().parse().unwrap();
    let val = &it.next().unwrap().parse().unwrap();
    (*index, *val)
}

fn parse_masks(mask: &str) -> Mask {
    let mut or_mask: usize = 0;
    let mut and_mask: usize = 0;
    let mask = mask.split_whitespace().skip(1).next().unwrap();
    for (bit, c) in mask.chars().enumerate() {
        if c == '1' {
            let m = 1 << 35 - bit;
            or_mask = or_mask | m;
        }
        if c != '0' {
            let m = 1 << 35 - bit;
            and_mask = and_mask | m;
        }
    }
    Mask {
        or_mask: or_mask,
        and_mask: and_mask,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mask_ptw_two_0_1_2() {
        let mut x_indexis: Vec<usize> = Vec::new();
        x_indexis.push(0);
        x_indexis.push(1);
        x_indexis.push(2);
        assert_eq!(2, or_mask_pt_two(2, &x_indexis));
    }

    #[test]
    fn test_mask_ptw_two_0_2() {
        let mut x_indexis: Vec<usize> = Vec::new();
        x_indexis.push(0);
        x_indexis.push(2);
        assert_eq!(5, or_mask_pt_two(3, &x_indexis));
    }
    #[test]
    fn test_mask_ptw_two_0_5() {
        let mut x_indexis: Vec<usize> = Vec::new();
        x_indexis.push(5);
        x_indexis.push(0);
        assert_eq!(1, or_mask_pt_two(1, &x_indexis));
    }

    #[test]
    fn test_parse_mask_and() {
        let mask = "mask 0101XX01X00X1X1011X1X000000101X10001";
        let expected_and = 0b010111011001111011111000000101110001;
        let mask = parse_masks(mask);
        println!("e_and {} m_and {}", expected_and, mask.and_mask);
        assert_eq!(expected_and, mask.and_mask);
    }

    #[test]
    fn test_parse_mask_or() {
        let mask = "mask 0101XX01X00X1X1011X1X000000101X10001";
        let expected_or = 0b010100010000101011010000000101010001;
        let mask = parse_masks(mask);
        println!("e_or {} mask_or {}", expected_or, mask.or_mask);
        assert_eq!(expected_or, mask.or_mask);
    }
}
