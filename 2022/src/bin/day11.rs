use std::{cell::RefCell, collections::VecDeque, num::ParseIntError, str::FromStr};

struct Monkey {
    items: RefCell<VecDeque<i64>>,
    items_inspected: RefCell<i64>,
    /// returns the operation on the worry level
    op: Box<dyn Fn(i64) -> i64>,
    /// returns the monkey id the item needs to be thrown to
    pred: Box<dyn Fn(i64) -> u8>,
}

impl std::fmt::Debug for Monkey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Monkey")
            // .field("items", &self.items)
            .field("items_inspected", &self.items_inspected)
            .finish()
    }
}

enum Operation {
    Add(Operand, Operand),
    Mul(Operand, Operand),
}

enum Operand {
    Constant(i64),
    Variable, // only 1 variable `old`
}

impl FromStr for Operand {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "old" => Ok(Operand::Variable),
            x => x.parse::<i64>().map(Operand::Constant),
        }
    }
}

fn process_in(s: &str) -> (Vec<Monkey>, i64) {
    s.split("\n\n")
        .map(process_monkey)
        .fold((vec![], 1), |mut acc, x| {
            if let Some(x) = x {
                acc.0.push(x.0);
                if acc.1 % x.1 != 0 {
                    acc.1 *= x.1;
                }
            }
            acc
        })
}

fn parse_eq(eq: &str) -> Option<Operation> {
    eq.split('=').map(|item| item.trim()).nth(1).and_then(|op| {
        let mut tokens = op.split(' ');
        let first_operand = tokens.next()?.parse::<Operand>().unwrap();
        let operator = tokens.next()?;
        let second_operand = tokens.next()?.parse::<Operand>().unwrap();
        match operator {
            "+" => Some(Operation::Add(first_operand, second_operand)),
            "*" => Some(Operation::Mul(first_operand, second_operand)),
            _ => None,
        }
    })
}

fn get_last_integer_in_line(s: &str) -> i64 {
    s.split(' ')
        .last()
        .map(|x| x.parse::<i64>())
        .unwrap()
        .unwrap()
}

fn process_monkey(s: &str) -> Option<(Monkey, i64)> {
    let mut it = s.lines();
    let _monkey_line = it.next()?;
    let items = it
        .next()?
        .split(':')
        .nth(1)
        .map(|items| {
            items
                .split(',')
                .map(|item| item.trim().parse::<i64>().unwrap())
                .collect::<VecDeque<_>>()
        })
        .unwrap();
    let op = it.next()?.split(':').nth(1).and_then(parse_eq).unwrap();
    let divisible_by = get_last_integer_in_line(it.next()?);
    let divisible_by_clone = divisible_by;
    let monkey_if_true = get_last_integer_in_line(it.next()?) as u8;
    let monkey_if_false = get_last_integer_in_line(it.next()?) as u8;
    Some((
        Monkey {
            items: RefCell::new(items),
            items_inspected: RefCell::new(0),
            op: match op {
                Operation::Add(Operand::Variable, Operand::Variable) => Box::new(move |x| x + x),
                Operation::Add(Operand::Variable, Operand::Constant(y)) => Box::new(move |x| x + y),
                Operation::Mul(Operand::Variable, Operand::Variable) => Box::new(move |x| x * x),
                Operation::Mul(Operand::Variable, Operand::Constant(y)) => Box::new(move |x| x * y),
                _ => unreachable!(""),
            },
            pred: Box::new(move |x| {
                if x % divisible_by == 0 {
                    monkey_if_true
                } else {
                    monkey_if_false
                }
            }),
        },
        divisible_by_clone,
    ))
}

fn round(monkeys: &[Monkey], mut subround: Box<dyn FnMut(i64) -> i64>) {
    for monkey in monkeys.iter() {
        loop {
            let mut items = monkey.items.borrow_mut();
            if items.is_empty() {
                break;
            }

            *monkey.items_inspected.borrow_mut() += 1;
            let item = items.pop_front().unwrap();
            // Note: we intentionally don't drop the items to exit early if we encounter infinite loop
            // i.e. the input data sends the item to the same monkey again.
            // drop(items);
            let worry = (monkey.op)(item);
            let worry = subround(worry);
            let monkey_id = (monkey.pred)(worry);
            monkeys[monkey_id as usize]
                .items
                .borrow_mut()
                .push_back(worry);
        }
    }
}

fn part_1(monkeys: &mut [Monkey]) -> i64 {
    for _ in 0..20 {
        round(monkeys, Box::new(|x| x / 3));
    }
    monkeys.sort_by_key(|x| -1 * *x.items_inspected.borrow());
    *monkeys[0].items_inspected.borrow() * *monkeys[1].items_inspected.borrow()
}

fn part_2(monkeys: &mut [Monkey], modulo: i64) -> i64 {
    for _ in 0..10000 {
        round(monkeys, Box::new(move |x| x % modulo));
    }
    monkeys.sort_by_key(|x| -1 * *x.items_inspected.borrow());
    *monkeys[0].items_inspected.borrow() * *monkeys[1].items_inspected.borrow()
}

fn main() {
    let s = std::fs::read_to_string("input/11.txt").expect("Unable to read file");
    let (mut monkeys, _modulo) = process_in(&s);
    let part_1 = part_1(&mut monkeys);
    let (mut monkeys, modulo) = process_in(&s);
    let part_2 = part_2(&mut monkeys, modulo);
    println!("Part 1: {}. Part 2: {}", part_1, part_2);
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Monkey 0:
    Starting items: 79, 98
    Operation: new = old * 19
    Test: divisible by 23
        If true: throw to monkey 2
        If false: throw to monkey 3

    Monkey 1:
    Starting items: 54, 65, 75, 74
    Operation: new = old + 6
    Test: divisible by 19
        If true: throw to monkey 2
        If false: throw to monkey 0

    Monkey 2:
    Starting items: 79, 60, 97
    Operation: new = old * old
    Test: divisible by 13
        If true: throw to monkey 1
        If false: throw to monkey 3

    Monkey 3:
    Starting items: 74
    Operation: new = old + 3
    Test: divisible by 17
        If true: throw to monkey 0
        If false: throw to monkey 1";

    #[test]
    fn test() {
        let (mut monkeys, modulo) = process_in(&INPUT);
        dbg!(modulo);
        assert!(part_1(&mut monkeys) == 10605);
        let (mut monkeys, modulo) = process_in(&INPUT);
        assert!(part_2(&mut monkeys, modulo) == 2713310158);
    }
}
