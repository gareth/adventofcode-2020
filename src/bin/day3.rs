use core::panic;
use std::fs;

pub fn main() {
    run("input/day3.txt");
}

fn run(filename: &str) {
    let contents = fs::read_to_string(filename).unwrap();

    let map = Map::new(&contents);

    println!("Trees encoutered: {}", map.navigate(3));
}

struct Map {
    width: usize,
    tiles: Vec<Vec<bool>>,
}

impl Map {
    /// Returns the number of trees encountered when navigating with a specific offset
    fn navigate(&self, offset: usize) -> i32 {
        let mut count = 0;
        for (i, row) in self.tiles.iter().enumerate() {
            let index = (i * offset) % self.width;
            if *row.get(index).unwrap() {
                count += 1
            }
        }
        count
    }
}

impl Map {
    fn new(data: &str) -> Map {
        let mut lines = data.lines().peekable();
        let width = lines.peek().expect("Expected first line").chars().count();
        let tiles = tiles(data, '#', width);

        Map { width, tiles }
    }
}

fn tiles(data: &str, needle: char, expect_width: usize) -> Vec<Vec<bool>> {
    data.lines()
        .map(|line| {
            if line.trim().chars().count() != expect_width {
                panic!("Expected line width {}", expect_width);
            }
            line.trim().chars().map(|c| c == needle).collect()
        })
        .collect::<Vec<Vec<bool>>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn construct_valid_map() {
        let map = Map::new("..#\n##.\n...\n");
        assert_eq!(
            vec![
                vec![false, false, true],
                vec![true, true, false],
                vec![false, false, false]
            ],
            map.tiles
        )
    }

    #[test]
    #[should_panic]
    fn panic_on_invalid_map() {
        Map::new("...\n..\n");
    }

    #[test]
    fn navigate() {
        let input = "\
..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";

        let map = Map::new(input);

        assert_eq!(7, map.navigate(3));
    }
}
