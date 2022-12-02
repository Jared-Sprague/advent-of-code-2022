use aoc_runner_derive::{aoc, aoc_generator};

pub struct Round {
    opponent_choice: String,
    your_choice: String,
}

impl Round {
    pub fn new(opponent_choice: String, input_b: String, is_part_2: bool) -> Self {
        let mut your_choice = input_b.clone();
        if is_part_2 {
            your_choice = make_choice(&opponent_choice, &input_b);
        }

        Self {
            opponent_choice,
            your_choice,
        }
    }

    fn get_points(&self) -> u32 {
        let mut points = 0u32;
        if self.is_win() {
            points += 6;
        } else if self.is_draw() {
            points += 3;
        }

        match self.your_choice.as_str() {
            "X" => points += 1, // rock
            "Y" => points += 2, // paper
            "Z" => points += 3, // scissors
            _ => panic!("unknown choice"),
        }

        points
    }

    fn is_win(&self) -> bool {
        match self.opponent_choice.as_str() {
            "A" => self.your_choice.as_str() == "Y",
            "B" => self.your_choice.as_str() == "Z",
            "C" => self.your_choice.as_str() == "X",
            _ => panic!("unknown choice"),
        }
    }

    fn is_draw(&self) -> bool {
        match self.opponent_choice.as_str() {
            "A" => self.your_choice.as_str() == "X",
            "B" => self.your_choice.as_str() == "Y",
            "C" => self.your_choice.as_str() == "Z",
            _ => panic!("unknown choice"),
        }
    }
}

pub fn make_choice(opponent_choice: &str, input_b: &str) -> String {
    // for part 2 your choice actually is the desired outcome so translate that
    match input_b {
        "X" => get_losing_choice(opponent_choice),  // lose
        "Y" => get_draw_choice(opponent_choice),    // draw
        "Z" => get_winning_choice(opponent_choice), // win
        _ => panic!("unknown choice"),
    }
}

pub fn get_winning_choice(opponent_choice: &str) -> String {
    match opponent_choice {
        "A" => String::from("Y"),
        "B" => String::from("Z"),
        "C" => String::from("X"),
        _ => panic!("unknown choice"),
    }
}

pub fn get_losing_choice(opponent_choice: &str) -> String {
    match opponent_choice {
        "A" => String::from("Z"),
        "B" => String::from("X"),
        "C" => String::from("Y"),
        _ => panic!("unknown choice"),
    }
}

pub fn get_draw_choice(opponent_choice: &str) -> String {
    match opponent_choice {
        "A" => String::from("X"),
        "B" => String::from("Y"),
        "C" => String::from("Z"),
        _ => panic!("unknown choice"),
    }
}

pub fn parse_rounds(input: &str, is_part_2: bool) -> Vec<Round> {
    let mut rounds: Vec<Round> = vec![];

    // Parse the input into vec of Rounds
    for line in input.lines() {
        let choices: Vec<&str> = line.split_whitespace().collect();
        let round = Round::new(
            choices.get(0).unwrap().to_string(),
            choices.get(1).unwrap().to_string(),
            is_part_2,
        );

        rounds.push(round);
    }

    rounds
}

#[aoc_generator(day2, part1)]
pub fn part1_gen(input: &str) -> Vec<Round> {
    parse_rounds(input, false)
}

#[aoc_generator(day2, part2)]
pub fn part2_gen(input: &str) -> Vec<Round> {
    parse_rounds(input, true)
}

#[aoc(day2, part1)]
pub fn part1(rounds: &[Round]) -> u32 {
    rounds.iter().map(|r| r.get_points()).sum()
}

#[aoc(day2, part2)]
pub fn part2(rounds: &[Round]) -> u32 {
    rounds.iter().map(|r| r.get_points()).sum()
}
