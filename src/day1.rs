use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Elf {
    total_calories: u32,
}

#[aoc_generator(day1)]
pub fn part1_gen(input: &str) -> Vec<Elf> {
    println!("part 1 generator");

    let mut elves: Vec<Elf> = vec![];

    let mut total = 0u32;

    for line in input.lines() {
        // println!("line: {}", line);

        if line.is_empty() {
            // println!("previous total: {}", total);
            let elf = Elf {
                total_calories: total,
            };
            elves.push(elf);
            total = 0;
        } else {
            let calories = line.parse::<u32>().unwrap();
            total += calories;
        }
    }

    elves.sort();
    elves.reverse();
    elves
}

#[aoc(day1, part1)]
pub fn part1(elves: &[Elf]) -> u32 {
    return elves.get(0).unwrap().total_calories;
}

#[aoc(day1, part2)]
pub fn part2(elves: &[Elf]) -> u32 {
    let mut total_top_three = 0u32;
    for i in 0..3 {
        total_top_three += elves.get(i).unwrap().total_calories;
    }
    total_top_three
}
