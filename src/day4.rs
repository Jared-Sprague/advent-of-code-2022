use aoc_runner_derive::{aoc, aoc_generator};

pub struct Pair {
    r1: (i32, i32),
    r2: (i32, i32),
}

impl Pair {
    fn new(line: &str) -> Self {
        // parse the ranges from the line
        let ranges: Vec<&str> = line.split(',').collect();
        let r1 = parse_range(ranges.get(0).unwrap().split('-').collect());
        let r2 = parse_range(ranges.get(1).unwrap().split('-').collect());
        Self { r1, r2 }
    }

    pub fn get_contains_count(&self) -> u32 {
        // see if one range contains another
        if (self.r1.0 <= self.r2.0 && self.r1.1 >= self.r2.1)
            || (self.r2.0 <= self.r1.0 && self.r2.1 >= self.r1.1)
        {
            return 1;
        }
        0
    }

    pub fn get_overlap_count(&self) -> u32 {
        // See if the ranges overlap at all
        if self.r1.1 >= self.r2.0 && self.r2.1 >= self.r1.0 {
            return 1;
        }
        0
    }
}

fn parse_range(range: Vec<&str>) -> (i32, i32) {
    (
        range.get(0).unwrap().parse::<i32>().unwrap(),
        range.get(1).unwrap().parse::<i32>().unwrap(),
    )
}

#[aoc_generator(day4)]
pub fn part1_gen(input: &str) -> Vec<Pair> {
    let mut pairs: Vec<Pair> = vec![];

    for line in input.lines() {
        let pair = Pair::new(line);
        pairs.push(pair);
    }

    pairs
}

#[aoc(day4, part1)]
pub fn part1(pairs: &[Pair]) -> u32 {
    pairs.iter().map(|p| p.get_contains_count()).sum()
}

#[aoc(day4, part2)]
pub fn part2(pairs: &[Pair]) -> u32 {
    pairs.iter().map(|p| p.get_overlap_count()).sum()
}
