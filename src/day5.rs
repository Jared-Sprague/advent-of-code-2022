use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;

pub struct Instruction {
    qty: u32,
    from: usize,
    to: usize,
}

#[aoc_generator(day5)]
pub fn part1_gen(input: &str) -> ([Vec<char>; 9], Vec<Instruction>) {
    let mut instructions: Vec<Instruction> = vec![];
    let stacks_re = Regex::new(
        r"^((\[(?P<s1>[A-Z])\])|(\s{3}))[ ]((\[(?P<s2>[A-Z])\])|(\s{3}))[ ]((\[(?P<s3>[A-Z])\])|(\s{3}))[ ]((\[(?P<s4>[A-Z])\])|(\s{3}))[ ]((\[(?P<s5>[A-Z])\])|(\s{3}))[ ]((\[(?P<s6>[A-Z])\])|(\s{3}))[ ]((\[(?P<s7>[A-Z])\])|(\s{3}))[ ]((\[(?P<s8>[A-Z])\])|(\s{3}))[ ]((\[(?P<s9>[A-Z])\])|(\s{3}))$",
    )
    .unwrap();
    let instructions_re = Regex::new(r"^move (?P<q>\d+) from (?P<f>\d) to (?P<t>\d)$").unwrap();
    let input_sections: Vec<&str> = input.split("\n\n").collect();
    let mut stacks: [Vec<char>; 9] = [
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
    ];

    // parse stacks section
    for line in input_sections[0].lines() {
        if line.starts_with(" 1") {
            break;
        }

        let caps = stacks_re.captures(line).unwrap();

        match caps.name("s1") {
            Some(c) => stacks[0].push(c.as_str().chars().next().unwrap()),
            None => {}
        }

        match caps.name("s2") {
            Some(c) => stacks[1].push(c.as_str().chars().next().unwrap()),
            None => {}
        }

        match caps.name("s3") {
            Some(c) => stacks[2].push(c.as_str().chars().next().unwrap()),
            None => {}
        }

        match caps.name("s4") {
            Some(c) => stacks[3].push(c.as_str().chars().next().unwrap()),
            None => {}
        }

        match caps.name("s5") {
            Some(c) => stacks[4].push(c.as_str().chars().next().unwrap()),
            None => {}
        }

        match caps.name("s6") {
            Some(c) => stacks[5].push(c.as_str().chars().next().unwrap()),
            None => {}
        }

        match caps.name("s7") {
            Some(c) => stacks[6].push(c.as_str().chars().next().unwrap()),
            None => {}
        }

        match caps.name("s8") {
            Some(c) => stacks[7].push(c.as_str().chars().next().unwrap()),
            None => {}
        }

        match caps.name("s9") {
            Some(c) => stacks[8].push(c.as_str().chars().next().unwrap()),
            None => {}
        }
    }

    stacks.iter_mut().for_each(|s| s.reverse());

    // parse instructions section
    for line in input_sections[1].lines() {
        let caps = instructions_re.captures(line).unwrap();
        let mut qty = 0;
        let mut from = 0;
        let mut to = 0;

        match caps.name("q") {
            Some(q) => qty = q.as_str().parse::<u32>().unwrap(),
            None => {}
        }

        match caps.name("f") {
            Some(f) => from = f.as_str().parse::<usize>().unwrap() - 1,
            None => {}
        }

        match caps.name("t") {
            Some(t) => to = t.as_str().parse::<usize>().unwrap() - 1,
            None => {}
        }

        instructions.push(Instruction { qty, from, to });
    }

    (stacks, instructions)
}

#[aoc(day5, part1)]
pub fn part1(input: &([Vec<char>; 9], Vec<Instruction>)) -> String {
    let mut stacks = input.0.clone();

    for instruction in input.1.iter() {
        for _ in 0..instruction.qty {
            let c = stacks[instruction.from].pop().unwrap();
            stacks[instruction.to].push(c);
        }
    }

    let mut answer = String::from("");
    for s in stacks.iter() {
        answer = format!("{}{}", answer, s.last().unwrap());
    }

    answer
}

// #[aoc(day5, part2)]
// pub fn part2(pairs: &[Pair]) -> u32 {
//     pairs.iter().map(|p| p.get_overlap_count()).sum()
// }
