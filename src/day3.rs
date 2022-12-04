use aoc_runner_derive::{aoc, aoc_generator};

static PRIORITY: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

pub struct Rucksack {
    all: Vec<char>,
    compartment_1: Vec<char>,
    compartment_2: Vec<char>,
}

impl Rucksack {
    fn new(items: &str) -> Self {
        let compartment_length = items.len() / 2;
        let slice1 = &items[..compartment_length];
        let slice2 = &items[compartment_length..];

        let all: Vec<char> = items.chars().collect();
        let compartment_1: Vec<char> = slice1.chars().collect();
        let compartment_2: Vec<char> = slice2.chars().collect();

        Self {
            all,
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
                dup_score += p;
                break;
            }
        }
        dup_score
    }

    pub fn contains(&self, item: &char) -> bool {
        self.all.contains(item)
    }
}

pub struct Group {
    sacks: (Rucksack, Rucksack, Rucksack),
}

impl Group {
    fn new(lines: (&str, &str, &str)) -> Self {
        Self {
            sacks: (
                Rucksack::new(lines.0),
                Rucksack::new(lines.1),
                Rucksack::new(lines.2),
            ),
        }
    }

    pub fn get_badge_score(&self) -> u32 {
        let mut badge_score = 0u32;
        for c in self.sacks.0.all.iter() {
            if self.sacks.1.contains(c) && self.sacks.2.contains(c) {
                let p = (PRIORITY.find(*c).unwrap() + 1) as u32;
                badge_score += p;
                break;
            }
        }
        badge_score
    }
}

#[aoc_generator(day3, part1)]
pub fn part1_gen(input: &str) -> Vec<Rucksack> {
    let mut rucksacks: Vec<Rucksack> = vec![];

    for line in input.lines() {
        let sack = Rucksack::new(line);
        rucksacks.push(sack);
    }

    rucksacks
}

#[aoc_generator(day3, part2)]
pub fn part2_gen(input: &str) -> Vec<Group> {
    let mut i = 0;
    let mut groups: Vec<Group> = vec![];
    let mut temp_sacks: Vec<&str> = vec![];

    for line in input.lines() {
        temp_sacks.push(line);
        i += 1;
        if i == 3 {
            groups.push(Group::new((
                temp_sacks.get(0).unwrap(),
                temp_sacks.get(1).unwrap(),
                temp_sacks.get(2).unwrap(),
            )));
            temp_sacks = vec![];
            i = 0;
        }
    }
    groups
}

#[aoc(day3, part1)]
pub fn part1(rucksacks: &[Rucksack]) -> u32 {
    rucksacks.iter().map(|f| f.get_duplicate_score()).sum()
}

#[aoc(day3, part2)]
pub fn part2(groups: &[Group]) -> u32 {
    groups.iter().map(|g| g.get_badge_score()).sum()
}
