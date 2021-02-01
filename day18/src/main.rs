use std::env;
use std::fs;
use std::str;

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
    let result: usize = content
        .lines()
        .map(|line| eval_homework(&line, eval_flattened))
        .sum();
    println!("part-one {}", result);
}

fn part_two(content: &str) {
    let result: usize = content
        .lines()
        .map(|line| eval_homework(&line, eval_flattened_part_two))
        .sum();
    println!("part-two {}", result);
}

fn eval_homework(str: &str, expression_eval: fn(&Vec<String>) -> usize) -> usize {
    eval_tokens(&tokenize(&str), expression_eval)
}

fn eval_tokens(tokens: &Vec<String>, expression_eval: fn(&Vec<String>) -> usize) -> usize {
    let mut last_open = false;
    let mut last_open_at = 0;
    let mut parsable: Vec<Vec<String>> = Vec::new();

    for (i, t) in tokens.iter().enumerate() {
        match t.as_str() {
            "(" => {
                last_open = true;
                last_open_at = i;
            }
            ")" => {
                if last_open {
                    parsable.push(tokens[last_open_at..i + 1].iter().cloned().collect());
                }
                last_open = false;
            }
            _ => (),
        }
    }

    if parsable.len() > 0 {
        let mut original_string = tokens.join("");
        for p in parsable {
            let result = expression_eval(&p);
            original_string = original_string.replace(&p.join(""), &result.to_string());
        }
        return eval_tokens(&tokenize(&original_string), expression_eval);
    }

    expression_eval(&tokens)
}

fn eval_flattened(expression: &Vec<String>) -> usize {
    let mut result = 0;
    let mut multiply = false;

    for t in expression {
        match t.as_str() {
            "(" | ")" => (),
            "*" => multiply = true,
            "+" => multiply = false,
            _ => {
                let numb = t.parse().unwrap();
                if result == 0 {
                    result = numb;
                    continue;
                }
                if multiply {
                    result *= numb;
                } else {
                    result += numb;
                }
            }
        }
    }
    result
}

fn eval_flattened_part_two(expression: &Vec<String>) -> usize {
    let mut joined = expression.join("");
    joined = joined.replace("(", "");
    joined = joined.replace(")", "");
    joined
        .split("*")
        .map(|part| eval_flattened(&tokenize(part)))
        .fold(1, |a, b| a * b)
}

fn tokenize(str: &str) -> Vec<String> {
    let mut token = Vec::new();
    let mut buffer = Vec::new();

    for char in str.chars() {
        if !char.is_digit(10) && buffer.len() > 0 {
            token.push(buffer.join(""));
            buffer.clear();
        }

        match char {
            '+' => token.push("+".to_string()),
            '*' => token.push("*".to_string()),
            '(' => token.push("(".to_string()),
            ')' => token.push(")".to_string()),
            ' ' => (),
            _ => buffer.push(char.to_string()),
        }
    }
    if buffer.len() > 0 {
        token.push(buffer.join(""));
    }
    token
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testpt1_1() {
        assert_eq!(
            51,
            eval_homework("1 + (2 * 3) + (4 * (5 + 6))", eval_flattened)
        );
    }

    #[test]
    fn testpt1_2() {
        assert_eq!(
            13632,
            eval_homework(
                "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2",
                eval_flattened
            )
        );
    }

    #[test]
    fn testpt2_1() {
        assert_eq!(
            51,
            eval_homework("1 + (2 * 3) + (4 * (5 + 6))", eval_flattened_part_two)
        );
    }

    #[test]
    fn tespt2_2() {
        assert_eq!(
            23340,
            eval_homework(
                "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2",
                eval_flattened_part_two
            )
        );
    }
}
