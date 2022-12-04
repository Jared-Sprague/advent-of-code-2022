use aoc_runner_derive::{aoc, aoc_generator};

static PRIORITY: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

pub struct Rucksack {
    compartment_1: Vec<char>,
    compartment_2: Vec<char>,
}

impl Rucksack {
    fn new(compartment_1: Vec<char>, compartment_2: Vec<char>) -> Self {
        let mut compartment_1 = compartment_1;
        let mut compartment_2 = compartment_2;
        compartment_1.sort();
        compartment_2.sort();

        Self {
            compartment_1,
            compartment_2,
        }
    }

    pub fn get_duplicate_score(&self) -> u32 {
        let mut dup_score = 0u32;
        for c in self.compartment_1.iter() {
            let dup = self.compartment_2.contains(c);
            if dup {
                let p = (PRIORITY.find(*c).unwrap() + 1) as u32;
                // println!("found dup: {} at: {}", c, p);
                dup_score += p;
                break;
            }
        }
        dup_score
    }
}

#[aoc_generator(day3)]
pub fn part1_gen(input: &str) -> Vec<Rucksack> {
    let mut rucksacks: Vec<Rucksack> = vec![];

    for line in input.lines() {
        let compartment_length = line.len() / 2;
        // println!("line: {}, {}", line, compartment_length);

        let c1 = &line[..compartment_length];
        let c2 = &line[compartment_length..];

        // println!("c1: {} c2: {}", c1, c2);

        let sack = Rucksack::new(c1.chars().collect(), c2.chars().collect());
        rucksacks.push(sack);
    }

    rucksacks
}

#[aoc(day3, part1)]
pub fn part1(rucksacks: &[Rucksack]) -> u32 {
    let mut total_dup_score = 0u32;
    for sack in rucksacks {
        total_dup_score += sack.get_duplicate_score();
    }
    total_dup_score
}

// #[aoc(day3, part2)]
// pub fn part2(rucksack: &[Rucksack]) -> u32 {
//     1
// }
