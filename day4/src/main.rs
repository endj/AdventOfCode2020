use regex::Regex;
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
    let parts = content.split("\n\n");
    let mut valid_count = 0;
    for part in parts {
        if part_one_is_valid(&part) {
            valid_count += 1;
        }
    }
    println!("valid part 1{}", valid_count);
}

fn part_one_is_valid(passport: &str) -> bool {
    let fields = vec!["ecl", "pid", "eyr", "hcl", "byr", "iyr", "hgt"];
    for field in fields {
        if !passport.contains(field) {
            return false;
        }
    }
    return true;
}

fn part_two(content: &String) {
    let parts = content.split("\n\n");
    let mut valid_count = 0;
    for part in parts {
        if part_two_is_valid(&part) {
            valid_count += 1;
        }
    }
    println!("valid part2 {}", valid_count);
}

fn part_two_is_valid(passport: &str) -> bool {
    let parts = passport.split_whitespace();
    let mut valid_fields_count = 0;
    for part in parts {
        let mut field = part.split(":");
        let key = field.next().unwrap();
        let value = field.next().unwrap();
        let valid = match key {
            "byr" => byr(value),
            "iyr" => iyr(value),
            "eyr" => eyr(value),
            "hgt" => hgt(value),
            "hcl" => hcl(value),
            "ecl" => ecl(value),
            "pid" => pid(value),
            _ => continue
        };
        if !valid {
            return false;
        } else {
            valid_fields_count += 1;
        }
    }
    return valid_fields_count == 7;
}

fn byr(year: &str) -> bool {
    year_check(&year, 1920, 2002)
}

fn iyr(year: &str) -> bool {
    year_check(&year, 2010, 2020)
}

fn eyr(year: &str) -> bool {
    year_check(&year, 2020, 2030)
}

fn hgt(height: &str) -> bool {
    if height.contains("cm") {
        let h: i32 = height.split("cm").next().unwrap().parse().unwrap();
        h >= 150 && h <= 193
    } else if height.contains("in") {
        let h: i32 = height.split("in").next().unwrap().parse().unwrap();
        h >= 59 && h <= 76
    } else {
        return false;
    }
}

fn hcl(hair: &str) -> bool {
    let re: Regex = Regex::new(r"^#([0-9a-f]{6})$").unwrap();
    re.is_match(hair)
}

// lazy_static todo maybe
fn pid(number: &str) -> bool {
    let re: Regex = Regex::new(r"^[0-9]{9}$").unwrap();
    re.is_match(number)
}

fn ecl(eye: &str) -> bool {
    let colors = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
    for color in colors {
        if eye == color {
            return true;
        }
    }
    return false;
}

fn year_check(year: &str, min: i32, max: i32) -> bool {
    let year_numb: i32 = year.parse().unwrap();
    year_numb >= min && year_numb <= max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_byr() {
        assert_eq!(byr("2002"), true);
        assert_eq!(byr("2003"), false);
    }

    #[test]
    fn test_hgt() {
        assert_eq!(hgt("60in"), true);
        assert_eq!(hgt("190cm"), true);
        assert_eq!(hgt("190in"), false);
        assert_eq!(hgt("190"), false);
    }

    #[test]
    fn test_hcl() {
        assert_eq!(hcl("#123abc"), true);
        assert_eq!(hcl("#123abz"), false);
        assert_eq!(hcl("123abc"), false);
    }

    #[test]
    fn test_ecl() {
        assert_eq!(ecl("brn"), true);
        assert_eq!(ecl("wat"), false);
    }

    #[test]
    fn test_pid() {
        assert_eq!(pid("000000001"), true);
        assert_eq!(pid("0123456789"), false);
    }
}
