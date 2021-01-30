use std::collections::HashSet;
use std::env;
use std::fs;

#[derive(Debug, Clone)]
struct Rule {
    min: usize,
    max: usize,
}

#[derive(Debug, Clone)]
struct FieldRule {
    name: String,
    first: Rule,
    second: Rule,
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
    let mut parts = content.trim().split("\n\n");

    let rules = parse_rules(parts.next().unwrap());
    let _my_ticket = parse_tickets(parts.next().unwrap()).pop().unwrap();
    let other_tickets = parse_tickets(parts.next().unwrap());

    let mut errors = 0;
    for ticket in other_tickets {
        errors += apply_rules(&rules, &ticket);
    }

    println!("part-one {}", errors);
}

fn part_two(content: &str) {
    let mut parts = content.trim().split("\n\n");
    let mut rules = parse_rules(parts.next().unwrap());
    let my_ticket = parse_tickets(parts.next().unwrap()).pop().unwrap();
    let fields = my_ticket.len();
    let other_tickets = parse_tickets(parts.next().unwrap());

    let valid_tickets: Vec<Vec<usize>> = other_tickets
        .into_iter()
        .filter(|ticket| is_valid(&rules, &ticket))
        .collect();

    let rule_count = rules.len();
    let mut assigned_count = 0;
    let mut assigned_fields: HashSet<usize> = HashSet::new();

    while assigned_count != rule_count {
        for field in 0..fields {
            if assigned_fields.contains(&field) {
                continue;
            }
            let mut posible_rule = None;
            let mut candidates = 0;

            for rule in &rules {
                let mut valid = true;
                for ticket in &valid_tickets {
                    if !in_bounds(&rule, &ticket[field]) {
                        valid = false;
                        break;
                    }
                }
                if valid {
                    candidates += 1;
                    posible_rule = Some(rule.clone());
                }
            }

            if candidates == 1 {
                let rule = posible_rule.unwrap();
                let index = &rules.iter().position(|x| *x.name == rule.name).unwrap();
                &rules.remove(*index);
                if rule.name.starts_with("departure") {
                    assigned_fields.insert(field);
                }
                assigned_count += 1;
            }
        }
    }
    let mut res = 1;
    for index in &assigned_fields {
        res *= &my_ticket[*index]
    }
    println!("part-two {}", res);
}

fn apply_rules(rules: &Vec<FieldRule>, ticket: &Vec<usize>) -> usize {
    let mut errors = 0;
    for value in ticket {
        errors += check_value(&rules, value)
    }
    errors
}

fn check_value(rules: &Vec<FieldRule>, value: &usize) -> usize {
    for rule in rules {
        if in_bounds(&rule, &value) {
            return 0;
        }
    }
    *value
}

fn is_valid(rules: &Vec<FieldRule>, ticket: &Vec<usize>) -> bool {
    apply_rules(&rules, &ticket) == 0
}

fn in_bounds(rule: &FieldRule, value: &usize) -> bool {
    rule.first.min <= *value && rule.first.max >= *value
        || rule.second.min <= *value && rule.second.max >= *value
}

fn parse_rules(str: &str) -> Vec<FieldRule> {
    str.lines().map(|l| parse_rule(&l)).collect()
}

fn parse_rule(str: &str) -> FieldRule {
    let mut parts = str.split(":");
    let name = parts.next().unwrap();
    let mut rules = parts.next().unwrap().split("or");
    let mut first = rules.next().unwrap().split("-");
    let mut second = rules.next().unwrap().split("-");
    FieldRule {
        name: name.to_string(),
        first: Rule {
            min: first.next().unwrap().trim().parse().unwrap(),
            max: first.next().unwrap().trim().parse().unwrap(),
        },
        second: Rule {
            min: second.next().unwrap().trim().parse().unwrap(),
            max: second.next().unwrap().trim().parse().unwrap(),
        },
    }
}

fn parse_tickets(str: &str) -> Vec<Vec<usize>> {
    str.lines()
        .skip(1)
        .map(|ticket| {
            ticket
                .split(",")
                .map(|c| c.parse().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bounds() {
        let rule = FieldRule {
            name: "abc".to_string(),
            first: Rule { min: 1, max: 3 },
            second: Rule { min: 5, max: 7 },
        };
        assert_eq!(true, in_bounds(&rule, &7));
        assert_eq!(false, in_bounds(&rule, &4));
        assert_eq!(false, in_bounds(&rule, &8));
    }
}
