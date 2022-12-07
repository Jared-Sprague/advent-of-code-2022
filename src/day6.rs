use std::collections::HashSet;

use aoc_runner_derive::{aoc, aoc_generator};

const TOKEN_LENGTH: usize = 14;

#[aoc_generator(day6)]
pub fn part1_gen(input: &str) -> String {
    String::from(input)
}

#[aoc(day6, part1)]
pub fn part1(input: &String) -> u32 {
    let chars: Vec<char> = input.chars().collect();
    for (n, _) in chars.iter().enumerate() {
        let mut marker: HashSet<char> = HashSet::new();

        //look ahead
        for j in 0..TOKEN_LENGTH {
            marker.insert(chars[n + j]);
        }

        if marker.len() == TOKEN_LENGTH {
            return (n + TOKEN_LENGTH) as u32;
        }
    }
    0
}
