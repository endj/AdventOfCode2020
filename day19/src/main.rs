use std::collections::HashMap;
use std::env;
use std::fs;

#[derive(Debug)]
struct Rule {
    id: usize,
    sub_rules: Option<Vec<Vec<usize>>>,
    letter: Option<String>,
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
    let mut parts = content.split("\n\n");

    let unparsed_rules = parts.next().unwrap();
    let unparsed_messages = parts.next().unwrap();

    let rules = parse_rules(&unparsed_rules);
    let messages = parse_messages(&unparsed_messages);

    let mut correct_messages = 0;
    for m in messages {
        let len = check_message(m, &rules, &0);
        if len == m.len() {
            correct_messages += 1;
        }
    }
    println!("part-one {}", correct_messages);
}
// Given
// 0: 1 2
// 1: "a"
// 2: 1 3 | 3 1
// 3: "b"
//
// And input aab
//
// (aab, 0) -> rule(aab, 0(1)) + rule( (aab - chars matched by previous rules)  0(2))

// rule(0(1)) -> 1
// letters matched = 1
// aab -> (a)ab -> ab

// rule(ab, 0(2)) -> rule(0(2(1))) -> 1
// letters matched = 1
// rule(0(2(3))) -> 1
// letter matched = 2
// ab -> ()
// Thus rule(aab, 0) -> 1 + 2 and since aab.len == 3 all 3 characters match rule

// 0: 1 | 1 2
fn check_message(message: &str, rules: &HashMap<usize, Rule>, rule_id: &usize) -> usize {
    let rule = rules.get(&rule_id).unwrap();

    if rule.letter.as_ref().is_some() {
        let letter = rule.letter.as_ref().unwrap();
        if message[..1] == *letter {
            return 1;
        } else {
            return 0;
        }
    }

    let mut letters_checked: usize = 0;
    for r in rule.sub_rules.as_ref().unwrap() {
        for r_id in r {
            let letter_matched = check_message(&message[letters_checked..], &rules, r_id);
            // If left path does not match, reset match counter and check other
            if letter_matched == 0 {
                letters_checked = 0;
                break;
            }
            letters_checked += letter_matched
        }
        if letters_checked != 0 {
            return letters_checked;
        }
    }
    letters_checked
}

// Given
// 0: 1 | 1 2
// 1: "a"
// 2: 1 3 | 3 1
// 3: "b"

// part-one rule wont work, consider aab will return 2 using same alg
// If we consider all options aab matches both options of 0, should give
// us that left matches with len 1 and right with len 3

fn check_message_pt2(message: &str, rules: &HashMap<usize, Rule>, rule_id: &usize) -> Vec<usize> {
    let rule = rules.get(&rule_id).unwrap();

    if rule.letter.as_ref().is_some() {
        let letter = rule.letter.as_ref().unwrap();
        if message.len() > 0 && message[..1] == *letter {
            return vec![1];
        } else {
            return vec![];
        }
    }

    let mut all_matching_lengths: Vec<usize> = Vec::new();
    for sub_rule in rule.sub_rules.as_ref().unwrap() {
        // For each option
        let mut matched: Vec<usize> = vec![0];

        for sub_rule_id in sub_rule {
            let mut acc: Vec<usize> = vec![];
            for match_len in matched {
                for c in check_message_pt2(&message[match_len..], &rules, sub_rule_id) {
                    acc.push(match_len + c);
                }
            }
            matched = acc;
        }

        all_matching_lengths.append(&mut matched)
    }
    all_matching_lengths
}

fn part_two(content: &str) {
    let mut parts = content.split("\n\n");

    let unparsed_rules = parts.next().unwrap();
    let unparsed_messages = parts.next().unwrap();

    let mut rules = parse_rules(&unparsed_rules);
    add_part_two_rules(&mut rules);
    let messages = parse_messages(&unparsed_messages);

    let mut correct_messages = 0;
    for m in messages {
        let lengths = check_message_pt2(m, &rules, &0);
        for i in lengths {
            if i == m.len() {
                correct_messages += 1;
            }
        }
    }
    println!("part-two {}", correct_messages);
}

fn add_part_two_rules(map: &mut HashMap<usize, Rule>) {
    map.insert(
        8,
        Rule {
            id: 8,
            sub_rules: Some(vec![vec![42], vec![42, 8]]),
            letter: None,
        },
    );
    map.insert(
        11,
        Rule {
            id: 11,
            sub_rules: Some(vec![vec![42, 31], vec![42, 11, 31]]),
            letter: None,
        },
    );
}

fn parse_rules(rules: &str) -> HashMap<usize, Rule> {
    let mut parsed_rules = HashMap::new();

    for rule in rules.lines() {
        let mut parts = rule.split(":");
        let id = parts.next().unwrap().parse().unwrap();
        let rest = parts.next().unwrap();

        let sub_rules: Vec<&str> = rest.trim().split("|").collect();

        if sub_rules.len() == 1 {
            let rule = parse_single_arg_rule(id, &sub_rules);
            parsed_rules.insert(id, rule);
        } else {
            let rule = parse_multiple_arg_rule(id, &sub_rules);
            parsed_rules.insert(id, rule);
        }
    }
    parsed_rules
}

fn parse_multiple_arg_rule(id: usize, sub_rules: &Vec<&str>) -> Rule {
    Rule {
        id: id,
        sub_rules: Some(parse_sub_rules(&sub_rules)),
        letter: None,
    }
}

fn parse_single_arg_rule(id: usize, sub_rules: &Vec<&str>) -> Rule {
    let rule_or_char = sub_rules.get(0).unwrap().replace("\"", "");
    if rule_or_char == "a" || rule_or_char == "b" {
        return Rule {
            id: id,
            sub_rules: None,
            letter: Some(rule_or_char.to_string()),
        };
    } else {
        return Rule {
            id: id,
            sub_rules: Some(parse_sub_rules(&sub_rules)),
            letter: None,
        };
    }
}

fn parse_sub_rules(rules: &Vec<&str>) -> Vec<Vec<usize>> {
    rules.iter().map(|rule| parse_sub_rule(rule)).collect()
}

fn parse_sub_rule(rule: &str) -> Vec<usize> {
    rule.split_whitespace()
        .map(|r| r.parse().unwrap())
        .collect()
}

fn parse_messages(messages: &str) -> Vec<&str> {
    messages.lines().collect()
}

#[cfg(test)]
mod tests {
    use super::*;
}
