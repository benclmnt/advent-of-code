use std::collections::VecDeque;

fn part_1(movements: &str, supplies: &mut Supplies) -> String {
    for mvmt in movements.lines() {
        process_movement_line_crane_9000(supplies, mvmt);
    }
    print_results(supplies)
}

fn part_2(movements: &str, supplies: &mut Supplies) -> String {
    for mvmt in movements.lines() {
        process_movement_line_crane_9001(supplies, mvmt);
    }
    print_results(supplies)
}

fn print_results(supplies: &Supplies) -> String {
    let mut result = String::from("");
    supplies.data.iter().for_each(|x| {
        if x.len() > 0 {
            result.push(x[0] as char);
        }
    });
    println!("{}", result);
    result
}

#[derive(Debug)]
struct Supplies {
    data: Vec<VecDeque<u8>>,
}

fn process_in(s: &str) -> (&str, Supplies) {
    let mut supplies = Supplies {
        data: vec![VecDeque::new(); 9],
    };
    let lines = s.split("\n\n").collect::<Vec<_>>();
    let mut crates = lines[0].lines().collect::<Vec<_>>();
    crates.pop(); // remove the number lines

    for line in crates {
        process_crate_line(&mut supplies, line);
    }

    (lines[1], supplies)
}

fn process_crate_line(supplies: &mut Supplies, s: &str) {
    for (i, c) in s.chars().enumerate() {
        if i % 4 == 1 && c != ' ' {
            supplies.data[(i - 1) / 4].push_back(c as u8);
        }
    }
}

fn process_movement_line_crane_9000(supplies: &mut Supplies, s: &str) {
    let tokens = s.split(" ").collect::<Vec<_>>();
    let count = tokens[1].parse::<u32>().unwrap();
    let from = tokens[3].parse::<u32>().unwrap() - 1;
    let to = tokens[5].parse::<u32>().unwrap() - 1;

    for _ in 0..count {
        let c = supplies.data[from as usize].pop_front().unwrap();
        supplies.data[to as usize].push_front(c);
    }
}

fn process_movement_line_crane_9001(supplies: &mut Supplies, s: &str) {
    let tokens = s.split(" ").collect::<Vec<_>>();
    let count = tokens[1].parse::<u32>().unwrap();
    let from = tokens[3].parse::<u32>().unwrap() - 1;
    let to = tokens[5].parse::<u32>().unwrap() - 1;

    let tmp = supplies.data[from as usize]
        .drain(0..count as usize)
        .collect::<VecDeque<_>>();
    for c in tmp.iter().rev() {
        supplies.data[to as usize].push_front(*c);
    }
}

fn main() {
    let data = std::fs::read_to_string("input/05.txt").expect("Unable to read file");
    let (movements, mut supplies) = process_in(&data);
    let part_1 = part_1(movements, &mut supplies);
    let (movements, mut supplies) = process_in(&data);
    let part_2 = part_2(movements, &mut supplies);
    println!("Part 1: {}. Part 2: {}", part_1, part_2);
}

mod tests {
    use super::*;

    const INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn test() {
        let (movements, mut supplies) = process_in(INPUT);
        assert!(part_1(movements, &mut supplies) == "CMZ");
        let (movements, mut supplies) = process_in(INPUT);
        assert!(part_2(movements, &mut supplies) == "MCD");
    }
}
