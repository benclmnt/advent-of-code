fn part_1(a: &[u32]) -> u32 {
    *a.iter().max().unwrap()
}

fn part_2(a: &mut [u32]) -> u32 {
    a.sort_by(|a, b| b.cmp(a));
    a.iter().take(3).sum()
}

fn process_in(input: &str) -> Vec<u32> {
    input
        .split("\n\n")
        .map(|elf| {
            elf.lines()
                .map(|item| item.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .collect::<Vec<_>>()
}

fn main() {
    // read a file
    let input = std::fs::read_to_string("input/01.txt").expect("Unable to read file");
    let mut data = process_in(&input);
    let part_1 = part_1(&data);
    let part_2 = part_2(&mut data);
    println!("Part 1: {}. Part 2: {}", part_1, part_2);
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn test() {
        let mut data: Vec<u32> = process_in(INPUT);

        assert!(part_1(&data) == 24000);
        assert!(part_2(&mut data) == 45000);
    }
}
