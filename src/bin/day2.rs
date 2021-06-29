#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::{fs, str::FromStr};

pub fn main() {
    run("input/day2.txt");
}

pub fn run(filename: &str) {
    let contents = fs::read_to_string(filename).unwrap();

    let matches = matching_entries::<CountPolicy>(&contents);
    println!("Matching entries (CountPolicy): {}", matches);

    let matches = matching_entries::<IndexPolicy>(&contents);
    println!("Matching entries (IndexPolicy): {}", matches);
}

fn matching_entries<T>(s: &str) -> usize
where
    T: Policy,
{
    s.lines()
        .filter(|line| {
            let entry = line.trim().parse::<Entry<T>>();
            entry.unwrap().valid()
        })
        .count()
}

pub struct Entry<T>
where
    T: Policy,
{
    policy: Box<T>,
    password: String,
}

impl<T> Entry<T>
where
    T: Policy,
{
    fn valid(&self) -> bool {
        self.policy.validate(&self.password)
    }
}

impl<T> FromStr for Entry<T>
where
    T: Policy,
{
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (policy_str, password) = s.split_once(": ").ok_or("Couldn't split input")?;
        let policy = policy_str.parse::<T>();

        match policy {
            Ok(p) => Ok(Entry {
                policy: Box::new(p),
                password: password.to_owned(),
            }),
            Err(_) => Err("No entry"),
        }
    }
}

pub trait Policy: FromStr {
    fn validate(&self, s: &str) -> bool;
}

pub struct CountPolicy {
    pub char: char,
    pub min: usize,
    pub max: usize,
}

impl Policy for CountPolicy {
    fn validate(&self, s: &str) -> bool {
        let actual = s.chars().into_iter().filter(|c| *c == self.char).count();

        actual >= self.min && actual <= self.max
    }
}

impl FromStr for CountPolicy {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (min, max, char) = parse_policy(s)?;
        Ok(CountPolicy { char, min, max })
    }
}

struct IndexPolicy {
    pub char: char,
    pub first: usize,
    pub second: usize,
}

impl Policy for IndexPolicy {
    fn validate(&self, s: &str) -> bool {
        let mut iter = s.chars();

        let first = iter.nth(self.first - 1).expect("No first char");

        let second = iter
            .nth(self.second - self.first - 1)
            .expect("No second char");

        (first == self.char) != (second == self.char)
    }
}

impl FromStr for IndexPolicy {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (first, second, char) = parse_policy(s)?;
        Ok(IndexPolicy {
            char,
            first,
            second,
        })
    }
}

/// Parse a policy string returning the two numbers and character
/// Expects a string in a compatible format: `1-2 f`
fn parse_policy(s: &str) -> Result<(usize, usize, char), &'static str> {
    lazy_static! {
        static ref RE: regex::Regex = regex::Regex::new(r"^(\d+)-(\d+)\s+(\w)$").unwrap();
    }

    let captures = RE.captures(s).unwrap();

    Ok((
        // TODO: Too many unwraps, be tidier
        captures.get(1).unwrap().as_str().parse::<usize>().unwrap(),
        captures.get(2).unwrap().as_str().parse::<usize>().unwrap(),
        captures.get(3).unwrap().as_str().chars().next().unwrap(),
    ))
}

#[cfg(test)]
mod tests {
    #[cfg(test)]
    mod count_policy {
        use super::super::*;

        #[test]
        fn parse_policy() {
            let policy: CountPolicy = "2-7 c".parse().unwrap();

            assert_eq!(policy.min, 2);
            assert_eq!(policy.max, 7);
            assert_eq!(policy.char, 'c');

            let policy: CountPolicy = "8-12 f".parse().unwrap();

            assert_eq!(policy.min, 8);
            assert_eq!(policy.max, 12);
            assert_eq!(policy.char, 'f');
        }

        #[test]
        fn parse_entry() {
            let entry: Entry<CountPolicy> = "1-2 a: foo".parse().unwrap();

            assert_eq!(entry.password, "foo");
        }

        #[test]
        fn entry_valid_with_exact_count() {
            let input = "1-1 x: x";

            let entry = input.parse::<Entry<CountPolicy>>().unwrap();

            assert!(entry.valid());
        }

        #[test]
        fn entry_invalid_with_too_few_instances() {
            let input = "2-2 x: x";

            let entry = input.parse::<Entry<CountPolicy>>().unwrap();

            assert!(!entry.valid());
        }

        #[test]
        fn entry_invalid_with_too_many_instances() {
            let input = "1-1 x: xx";

            let entry = input.parse::<Entry<CountPolicy>>().unwrap();

            assert!(!entry.valid());
        }
    }

    mod index_policy {
        use super::super::*;

        #[test]
        fn parse_policy() {
            let policy: IndexPolicy = "2-7 c".parse().unwrap();

            assert_eq!(policy.first, 2);
            assert_eq!(policy.second, 7);
            assert_eq!(policy.char, 'c');

            let policy: IndexPolicy = "8-12 f".parse().unwrap();

            assert_eq!(policy.first, 8);
            assert_eq!(policy.second, 12);
            assert_eq!(policy.char, 'f');
        }

        #[test]
        fn parse_entry() {
            let entry: Entry<IndexPolicy> = "1-2 a: foo".parse().unwrap();

            assert_eq!(entry.password, "foo");
        }

        #[test]
        fn entry_valid_with_correct_indexes() {
            let input = "1-3 a: abcde";

            let entry = input.parse::<Entry<IndexPolicy>>().unwrap();

            assert!(entry.valid());
        }

        #[test]
        fn entry_invalid_with_no_matching_chars() {
            let input = "1-3 b: cdefg";

            let entry = input.parse::<Entry<IndexPolicy>>().unwrap();

            assert!(!entry.valid());
        }

        #[test]
        fn entry_invalid_with_two_matching_chars() {
            let input = "2-9 c: ccccccccc";

            let entry = input.parse::<Entry<IndexPolicy>>().unwrap();

            assert!(!entry.valid());
        }
    }
}
