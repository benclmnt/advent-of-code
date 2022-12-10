struct Instruction {
    inst_type: InstructionType,
    times_called: u8,
}

enum InstructionType {
    Noop,
    AddX(i32),
}

impl Instruction {
    fn new(inst_type: InstructionType) -> Self {
        Self {
            inst_type,
            times_called: 0,
        }
    }
}

impl Iterator for Instruction {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        match self.inst_type {
            InstructionType::Noop => {
                if self.times_called > 0 {
                    None
                } else {
                    self.times_called = 1;
                    Some(0)
                }
            }
            InstructionType::AddX(x) => match self.times_called {
                0 => {
                    self.times_called = 1;
                    Some(0)
                }
                1 => {
                    self.times_called = 2;
                    Some(x)
                }
                _ => None,
            },
        }
    }
}

fn part_1(s: &str) -> i32 {
    let mut it = s.lines().flat_map(|line| {
        let mut it = line.split(" ");
        let inst = it.next().unwrap();
        if inst == "noop" {
            Instruction::new(InstructionType::Noop)
        } else if inst == "addx" {
            let x = it.next().unwrap().parse::<i32>().unwrap();
            Instruction::new(InstructionType::AddX(x))
        } else {
            unreachable!()
        }
    });

    let mut cycle = 0;
    let mut register_x = 1; // initial value of X
    let mut sum = 0;

    loop {
        let next = it.next();
        cycle += 1;
        if next.is_none() {
            break;
        }
        if cycle % 40 == 20 {
            sum += register_x * cycle;
        }
        // end of cycle
        register_x += next.unwrap();
    }
    sum
}

fn part_2(s: &str) -> String {
    let mut it = s.lines().flat_map(|line| {
        let mut it = line.split(" ");
        let inst = it.next().unwrap();
        if inst == "noop" {
            Instruction::new(InstructionType::Noop)
        } else if inst == "addx" {
            let x = it.next().unwrap().parse::<i32>().unwrap();
            Instruction::new(InstructionType::AddX(x))
        } else {
            unreachable!()
        }
    });

    let mut cycle = 0;
    let mut register_x = 1; // initial value of X
    let mut s = String::new();

    loop {
        let next = it.next();
        cycle += 1;
        if next.is_none() {
            break;
        }

        s.push(
            if register_x - 1 <= (cycle - 1) % 40 && (cycle - 1) % 40 <= register_x + 1 {
                '#'
            } else {
                '.'
            },
        );
        if cycle % 40 == 0 {
            s.push('\n');
        }

        // end of cycle
        register_x += next.unwrap();
    }
    print!("{}", &s);
    s
}

fn main() {
    let movements = std::fs::read_to_string("input/10.txt").expect("Unable to read file");
    let part_1 = part_1(&movements);
    let part_2 = part_2(&movements);
    println!("Part 1: {}. Part 2: {}", part_1, part_2);
}

mod tests {
    use super::*;

    const INPUT: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

    #[test]
    fn test() {
        assert!(part_1(&INPUT) == 13140);
        assert!(
            part_2(&INPUT)
                == "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
" // note the addition of extra newline at the end
        );
    }
}
