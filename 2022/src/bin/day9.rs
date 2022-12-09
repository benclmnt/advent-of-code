use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

fn move_head(head: &mut Position, dir: &str) {
    match dir {
        "R" => head.x += 1,
        "L" => head.x -= 1,
        "U" => head.y += 1,
        "D" => head.y -= 1,
        _ => unreachable!(),
    }
}

fn move_tail(head: &Position, tail: &mut Position) {
    if tail.y.abs_diff(head.y) == 2 {
        tail.y = (head.y + tail.y) / 2;
        tail.x = if tail.x.abs_diff(head.x) < 2 {
            head.x
        } else {
            (head.x + tail.x) / 2
        }
    } else if tail.x.abs_diff(head.x) == 2 {
        tail.x = (head.x + tail.x) / 2;
        tail.y = if tail.y.abs_diff(head.y) < 2 {
            head.y
        } else {
            (head.y + tail.y) / 2
        }
    }
}

fn part_1(s: &str) -> u32 {
    let mut visited = HashSet::with_capacity(2000);
    let mut head = Position { x: 0, y: 0 };
    let mut tail = Position { x: 0, y: 0 };
    visited.insert(tail.clone());

    for line in s.lines() {
        let mut it = line.split(" ");
        let dir = it.next().unwrap();
        let distance = it.next().map(|x| x.parse::<i8>().unwrap()).unwrap();

        for _ in 0..distance {
            move_head(&mut head, dir);
            move_tail(&head, &mut tail);
            visited.insert(tail.clone());
        }
    }

    visited.len() as u32
}

fn part_2(s: &str) -> u32 {
    let mut visited = HashSet::with_capacity(2000);
    let mut knots = [Position { x: 0, y: 0 }; 10];
    visited.insert(Position { x: 0, y: 0 });

    for line in s.lines() {
        let mut it = line.split(" ");
        let dir = it.next().unwrap();
        let distance = it.next().map(|x| x.parse::<i8>().unwrap()).unwrap();

        for _ in 0..distance {
            move_head(&mut knots[0], dir);
            for i in 1..knots.len() {
                move_tail(&knots[i - 1].clone(), &mut knots[i]);
            }
            visited.insert(knots[knots.len() - 1].clone());
        }
    }

    visited.len() as u32
}

fn main() {
    let movements = std::fs::read_to_string("input/09.txt").expect("Unable to read file");
    let part_1 = part_1(&movements);
    let part_2 = part_2(&movements);
    println!("Part 1: {}. Part 2: {}", part_1, part_2);
}

mod tests {
    use super::*;

    const INPUT: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    const INPUT2: &str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

    #[test]
    fn test() {
        assert!(part_1(&INPUT) == 13);
        assert!(part_2(&INPUT) == 1);
        assert!(part_2(&INPUT2) == 36);
    }
}
