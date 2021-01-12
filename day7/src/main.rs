use std::collections::HashMap;
use std::env;
use std::fs;

#[derive(Debug)]
struct Bag {
    bag_type: String,
    sub_types: Vec<SubBag>,
}

#[derive(Debug)]
struct SubBag {
    bag_type: String,
    count: u32,
}

fn main() {
    let file_name = env::args()
        .skip(1)
        .next()
        .expect("expected input filepath argument");
    let content = fs::read_to_string(file_name)
        .unwrap()
        .replace("bags", "")
        .replace("bag", "")
        .replace("contain", "")
        .replace(".", "")
        .replace(",", "")
        .replace("no", "")
        .replace("other", "");

    part_one(&content);
    part_two(&content);
}
fn part_one(content: &String) {
    let mut bag_map: HashMap<String, Vec<SubBag>> = HashMap::new();

    let lines = content.trim().split("\n");
    for bag in lines {
        let parsed_bag: Bag = parse_bag(&bag);
        bag_map.insert(parsed_bag.bag_type, parsed_bag.sub_types);
    }

    let mut seen: HashMap<String, u32> = HashMap::new();
    let mut count = 0;

    for bag in bag_map.keys() {
        if holds_gold_bag(&bag, &bag_map, &mut seen) {
            count += 1;
        }
    }

    println!("part-one {}", count);
}

fn part_two(content: &String) {
    let mut bag_map: HashMap<String, Vec<SubBag>> = HashMap::new();

    let lines = content.trim().split("\n");
    for bag in lines {
        let parsed_bag: Bag = parse_bag(&bag);
        bag_map.insert(parsed_bag.bag_type, parsed_bag.sub_types);
    }

    println!("part-two {}", gold_bag_sub_bags("shinygold", &bag_map));
}

fn gold_bag_sub_bags(bag: &str, bags: &HashMap<String, Vec<SubBag>>) -> u32 {
    let sub_bags: &Vec<SubBag> = &*bags.get(bag).unwrap();
    let mut count = 0;
    for sub_bag in sub_bags {
        count += sub_bag.count;
        count += sub_bag.count * gold_bag_sub_bags(&sub_bag.bag_type, &bags);
    }
    return count;
}

fn holds_gold_bag(
    bag: &str,
    bags: &HashMap<String, Vec<SubBag>>,
    mut check: &mut HashMap<String, u32>,
) -> bool {
    if bag == "shinygold" {
        return false;
    }
    if check.contains_key(bag) {
        return true;
    }

    let sub_bags: &Vec<SubBag> = &*bags.get(bag).unwrap();
    for sub_bag in sub_bags {
        if sub_bag.bag_type == "shinygold" || holds_gold_bag(&sub_bag.bag_type, &bags, &mut check) {
            check.insert(bag.to_string(), 0);
            return true;
        }
    }
    return false;
}

fn parse_bag(bag: &str) -> Bag {
    let mut parts = bag.split_whitespace();
    let bag_type = format!("{}{}", parts.next().unwrap(), parts.next().unwrap());

    let mut sub_types: Vec<SubBag> = Vec::new();
    let mut peekable = parts.peekable();

    while peekable.peek().is_some() {
        let count: u32 = peekable.next().unwrap().parse().unwrap();
        let sub_type = format!("{}{}", peekable.next().unwrap(), peekable.next().unwrap());
        sub_types.push(SubBag {
            bag_type: sub_type,
            count: count,
        });
    }
    Bag {
        bag_type: bag_type,
        sub_types: sub_types,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_bag() {
        let b: Bag = parse_bag("light red   1 bright white  2 muted yellow");
        assert_eq!(b.bag_type, "lightred".to_string());
        assert_eq!(b.sub_types.len(), 2)
    }
}
