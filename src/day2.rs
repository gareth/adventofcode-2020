use regex;
use std::{fs, str::FromStr};

pub struct Policy {
    pub char: char,
    pub min: usize,
    pub max: usize,
}

impl FromStr for Policy {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: regex::Regex = regex::Regex::new(r"(\d+)-(\d+)\s+(\w)").unwrap();
        }
        // let re: regex::Regex = regex::Regex::new(r"(\d+)-(\d+)\s+(\w)").unwrap();

        if let Some(captures) = RE.captures(s) {
            Ok(Policy {
                char: captures.get(3).unwrap().as_str().chars().next().unwrap(),
                min: captures.get(1).unwrap().as_str().parse::<usize>().unwrap(),
                max: captures.get(2).unwrap().as_str().parse::<usize>().unwrap(),
            })
        } else {
            let error = format!("Policy not found in {}", s);
            Err(error)
        }
    }
}

pub struct Entry {
    policy: Policy,
    password: String,
}

impl Entry {
    fn valid(&self) -> bool {
        let policy = &self.policy;
        let actual = self
            .password
            .chars()
            .into_iter()
            .filter(|c| *c == policy.char)
            .count();

        actual >= policy.min && actual <= policy.max
    }
}

impl FromStr for Entry {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (policy, password) = s.split_once(": ").ok_or("Couldn't split input")?;

        Ok(Entry {
            policy: policy.parse().unwrap(),
            password: String::from(password),
        })
    }
}

pub fn run(filename: &str) {
    let contents = fs::read_to_string(filename).unwrap();
    let matches = contents
        .lines()
        .filter(|line| {
            let entry = line.trim().parse::<Entry>();
            entry.unwrap().valid()
        })
        .count();
    println!("Matching entries: {}", matches);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_policy() {
        let policy: Policy = "2-7 c".parse().unwrap();

        assert_eq!(policy.min, 2);
        assert_eq!(policy.max, 7);
        assert_eq!(policy.char, 'c');

        let policy: Policy = "8-12 f".parse().unwrap();

        assert_eq!(policy.min, 8);
        assert_eq!(policy.max, 12);
        assert_eq!(policy.char, 'f');
    }

    #[test]
    fn parse_entry() {
        let entry: Entry = "1-2 a: foo".parse().unwrap();

        assert_eq!(entry.password, "foo");
    }

    #[test]
    fn entry_valid_with_exact_count() {
        let input = "1-1 x: x";

        let entry = input.parse::<Entry>().unwrap();

        assert!(entry.valid());
    }

    #[test]
    fn entry_invalid_with_too_few_instances() {
        let input = "2-2 x: x";

        let entry = input.parse::<Entry>().unwrap();

        assert!(!entry.valid());
    }

    #[test]
    fn entry_invalid_with_too_many_instances() {
        let input = "1-1 x: xx";

        let entry = input.parse::<Entry>().unwrap();

        assert!(!entry.valid());
    }
}
