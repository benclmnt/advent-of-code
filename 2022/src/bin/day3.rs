use std::collections::HashSet;

fn part_1(data: &str) -> u32 {
    data.lines().map(get_similar_part1).sum()
}

fn part_2(data: &str) -> u32 {
    let mut i = 0;
    let mut tmp = ["", ""];
    let mut sum = 0;
    for line in data.lines() {
        if i == 2 {
            i = 0;
            sum += get_similar_part2(tmp[0], tmp[1], line);
        } else {
            tmp[i] = line;
            i += 1;
        }
    }
    sum
}

fn priority(c: char) -> u8 {
    match c {
        'a'..='z' => (c as u8) - b'a' + 1,
        'A'..='Z' => (c as u8) - b'A' + 27,
        _ => 0,
    }
}

fn get_similar_part1(items: &str) -> u32 {
    let n = items.len();
    let first: HashSet<_> = items.chars().take(n / 2).map(|x| x as u8).collect();
    let second: HashSet<_> = items.chars().skip(n / 2).map(|x| x as u8).clone().collect();
    let intersection = &first & &second;
    intersection
        .into_iter()
        .map(|x| priority(x as char) as u32)
        .sum()
}

fn get_similar_part2(a: &str, b: &str, c: &str) -> u32 {
    let ah: HashSet<_> = a.chars().map(|x| x as u8).collect();
    let bh: HashSet<_> = b.chars().map(|x| x as u8).collect();
    let ch: HashSet<_> = c.chars().map(|x| x as u8).collect();
    let intersection = &(&ah & &bh) & &ch;
    intersection
        .into_iter()
        .map(|x| priority(x as char) as u32)
        .sum::<u32>()
}
fn main() {
    let data = std::fs::read_to_string("input/03.txt").expect("Unable to read file");
    let part_1 = part_1(&data);
    let part_2 = part_2(&data);
    println!("Part 1: {}. Part 2: {}", part_1, part_2);
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn test_priority() {
        assert!(priority('c') == 3);
        assert!(priority('L') == 38);
    }

    #[test]
    fn test_get_similar() {
        assert!(get_similar_part1("vJrwpWtwJgWrhcsFMMfFFhFp") == 16); // p
        assert!(get_similar_part1("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL") == 38); // L

        assert!(
            get_similar_part2(
                "vJrwpWtwJgWrhcsFMMfFFhFp",
                "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
                "PmmdzqPrVvPwwTWBwg"
            ) == 18 // r
        );
    }

    #[test]
    fn test() {
        assert!(part_1(INPUT) == 157);
        assert!(part_2(INPUT) == 70);
    }
}
