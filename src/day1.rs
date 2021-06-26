use std::fs;

pub fn run(filename: &str) {
    let values = integers(filename);

    match pairs(&values, 2020) {
        Ok(result) => {
            println!("{}", result)
        }
        Err(_) => {
            println!("No pair found")
        }
    }

    match triplets(&values, 2020) {
        Ok(result) => {
            println!("{}", result)
        }
        Err(_) => {
            println!("No triplet found")
        }
    }
}

fn integers(filename: &str) -> Vec<u32> {
    let contents = fs::read_to_string(filename).unwrap();
    let values: Vec<u32> = contents
        .lines()
        .into_iter()
        .map(|line| line.parse::<u32>().expect("Unable to parse as a number"))
        .collect();
    values
}

pub fn pairs(values: &Vec<u32>, target: u32) -> Result<u32, &'static str> {
    for (i, x) in values[..].into_iter().enumerate() {
        for y in values[i..].into_iter() {
            if x + y == target {
                return Ok(x * y);
            }
        }
    }
    Err("No suitable pair found")
}

pub fn triplets(values: &Vec<u32>, target: u32) -> Result<u32, &'static str> {
    for (i, x) in values[..].into_iter().enumerate() {
        for (j, y) in values[i..].into_iter().enumerate() {
            for z in values[(i + j)..].into_iter() {
                if x + y + z == target {
                    return Ok(x * y * z);
                }
            }
        }
    }
    Err("No suitable pair found")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_pair() {
        let input: Vec<u32> = vec![1721, 979, 366, 299, 675, 1456];

        assert_eq!(514579, pairs(&input, 2020).unwrap())
    }

    #[test]
    fn a_triplet() {
        let input: Vec<u32> = vec![1721, 979, 366, 299, 675, 1456];

        assert_eq!(241861950, triplets(&input, 2020).unwrap())
    }
}
