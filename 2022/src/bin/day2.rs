use std::cmp::Ordering;

#[derive(PartialEq, Copy, Clone, Debug)]
#[repr(u8)]
enum Move {
    Rock = 1,
    Paper,
    Scissor,
}

impl From<&str> for Move {
    fn from(n: &str) -> Self {
        match n {
            "A" | "X" => Move::Rock,
            "B" | "Y" => Move::Paper,
            "C" | "Z" => Move::Scissor,
            _ => panic!("Invalid move"),
        }
    }
}

impl std::cmp::PartialOrd for Move {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Move::Rock, Move::Paper) => Some(Ordering::Greater),
            (Move::Rock, Move::Scissor) => Some(Ordering::Less),
            (Move::Paper, Move::Rock) => Some(Ordering::Less),
            (Move::Paper, Move::Scissor) => Some(Ordering::Greater),
            (Move::Scissor, Move::Rock) => Some(Ordering::Greater),
            (Move::Scissor, Move::Paper) => Some(Ordering::Less),
            _ => Some(Ordering::Equal),
        }
    }
}

fn part_1(input: &str) -> u32 {
    input
        .lines()
        .map(|strat| {
            let x = strat
                .split(" ")
                .map(|m| Move::from(m)).collect::<Vec<_>>();
            (x[0], x[1])
        })
        .map(|res| 
            match res.0.partial_cmp(&res.1) {
                Some(std::cmp::Ordering::Equal) => 3,
                Some(std::cmp::Ordering::Greater) => 6,
                _ => 0,
            } + res.1 as u32
        )
        .sum()
}

fn part_2(input: &str) -> u32 {
    input
    .lines()
    .map(|strat| {
        let x = strat
            .split(" ").collect::<Vec<_>>();
        match &x[..] {
            ["A", "X"] => (Move::Rock, Move::Scissor),
            ["A", "Y"] => (Move::Rock, Move::Rock),
            ["A", "Z"] => (Move::Rock, Move::Paper),
            ["B", "X"] => (Move::Paper, Move::Rock),
            ["B", "Y"] => (Move::Paper, Move::Paper),
            ["B", "Z"] => (Move::Paper, Move::Scissor),
            ["C", "X"] => (Move::Scissor, Move::Paper),
            ["C", "Y"] => (Move::Scissor, Move::Scissor),
            ["C", "Z"] => (Move::Scissor, Move::Rock),
            _ => panic!("Invalid move"),
        }
    })
    .map(|res| 
        match res.0.partial_cmp(&res.1) {
            Some(Ordering::Equal) => 3,
            Some(Ordering::Greater) => 6,
            _ => 0,
        } + res.1 as u32
    )
    .sum()
}

fn main() {
    // read a file
    let input = std::fs::read_to_string("input/02.txt").expect("Unable to read file");
    let part_1 = part_1(&input);
    let part_2 = part_2(&input);
    println!("Part 1: {}. Part 2: {}", part_1, part_2);
}

mod tests {
    use super::*;

    const INPUT: &str = "A Y
B X
C Z";

    #[test]
    fn test() {
        assert!(part_1(INPUT) == 15);
        assert!(part_2(INPUT) == 12);
    }
}
