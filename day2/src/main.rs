use prelude::*;
use std::str::FromStr;

#[derive(Debug)]
struct Rule {
    n1: u16,
    n2: u16,
    ch: char,
}
impl FromStr for Rule {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<String> = s.split("-").map(|x| x.to_string()).collect();
        let n1: &str = &parts[0];
        let parts: Vec<String> = parts[1].split(" ").map(|x| x.to_string()).collect();
        let n2: &str = &parts[0];
        let ch: &str = &parts[1];

        let n1: u16 = n1.parse().map_err(stringify_err)?;
        let n2: u16 = n2.parse().map_err(stringify_err)?;
        let ch: char = ch.parse().map_err(stringify_err)?;
        Ok(Rule { n1, n2, ch })
    }
}
impl Rule {
    fn is_valid_part1(&self, s: &str) -> bool {
        let count = s.matches(self.ch).count() as u16;
        count >= self.n1 && count <= self.n2
    }

    fn is_valid_part2(&self, s: &str) -> bool {
        let first = s.chars().nth(self.n1 as usize - 1) == Some(self.ch);
        let second = s.chars().nth(self.n2 as usize - 1) == Some(self.ch);
        first != second
    }
}

#[derive(Debug)]
struct PasswordEntry {
    pub rule: Rule,
    pub password: String,
}

impl FromStr for PasswordEntry {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<String> = s.split(": ").map(|x| x.to_string()).collect();
        let rule = parts[0].parse::<Rule>()?;
        Ok(PasswordEntry {
            rule,
            password: parts[1].clone(),
        })
    }
}

fn main() {
    let password_entries: Vec<PasswordEntry> = read_input_lines("input.txt")
        .into_iter()
        .map(|x| {
            x.parse::<PasswordEntry>()
                .expect("Failed to parse expense report entry")
        })
        .collect();

    println!("Part 1");
    let invalid_passwords_count_1: usize = password_entries
        .iter()
        .filter_map(|x| {
            if x.rule.is_valid_part1(&x.password) {
                Some(x)
            } else {
                None
            }
        })
        .count();
    println!("Answer: {}", invalid_passwords_count_1);
    println!("=======================");
    println!("Part 2");
    let invalid_passwords_count_2: usize = password_entries
        .iter()
        .filter_map(|x| {
            if x.rule.is_valid_part2(&x.password) {
                Some(x)
            } else {
                None
            }
        })
        .count();
    println!("Answer: {}", invalid_passwords_count_2);
}

#[cfg(test)]
mod tests {
    use super::PasswordEntry;

    fn get_inputs() -> Vec<PasswordEntry> {
        "1-3 a: abcde
        1-3 b: cdefg
        2-9 c: ccccccccc"
            .lines()
            .into_iter()
            .map(|x| {
                x.trim()
                    .parse::<PasswordEntry>()
                    .expect("Failed to parse expense report entry")
            })
            .collect()
    }
    #[test]
    fn part1() {
        let inputs = get_inputs();
        assert_eq!(true, inputs[0].rule.is_valid_part1(&inputs[0].password));
        assert_eq!(false, inputs[1].rule.is_valid_part1(&inputs[1].password));
        assert_eq!(true, inputs[2].rule.is_valid_part1(&inputs[2].password));
    }
    #[test]
    fn part2() {
        let inputs = get_inputs();
        assert_eq!(true, inputs[0].rule.is_valid_part2(&inputs[0].password));
        assert_eq!(false, inputs[1].rule.is_valid_part2(&inputs[1].password));
        assert_eq!(false, inputs[2].rule.is_valid_part2(&inputs[2].password));
    }
}
