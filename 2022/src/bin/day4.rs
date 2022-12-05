fn part_1(data: &Vec<Vec<Vec<u8>>>) -> u32 {
    data.iter().map(is_fully_contained).sum::<u32>()
}

/// is_fully_contained returned 1 if any of the ranges is contained in the other range and 0 otherwise
fn is_fully_contained(ranges: &Vec<Vec<u8>>) -> u32 {
    1 - (((ranges[0][0] < ranges[1][0]) && (ranges[0][1] < ranges[1][1]))
        || ((ranges[0][0] > ranges[1][0]) && (ranges[0][1] > ranges[1][1]))) as u32
}

/// overlaps returned 1 if the ranges overlaps and 0 otherwise
fn overlaps(ranges: &Vec<Vec<u8>>) -> u32 {
    1 - ((ranges[0][0] > ranges[1][1]) || (ranges[0][1] < ranges[1][0])) as u32
}

fn part_2(data: &Vec<Vec<Vec<u8>>>) -> u32 {
    data.iter().map(overlaps).sum::<u32>()
}

fn process_in(s: &str) -> Vec<Vec<Vec<u8>>> {
    s.lines()
        .map(|line| {
            line.split(",")
                .map(|range| {
                    range
                        .split("-")
                        .map(|i| i.parse::<u8>().unwrap())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn main() {
    let data = std::fs::read_to_string("input/04.txt").expect("Unable to read file");
    let input = process_in(&data);
    let part_1 = part_1(&input);
    let part_2 = part_2(&input);
    println!("Part 1: {}. Part 2: {}", part_1, part_2);
}

mod tests {
    use super::*;

    const INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn test_is_overlapping() {
        println!("{}", is_fully_contained(&vec![vec![2, 4], vec![2, 3]]));
    }

    #[test]
    fn test() {
        let input = process_in(INPUT);
        assert!(part_1(&input) == 2);
        assert!(part_2(&input) == 4);
    }
}
