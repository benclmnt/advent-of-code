use std::collections::HashSet;

fn solve(s: &str, marker_length: usize) -> u32 {
    let s = s.as_bytes();
    let mut start = 0;
    let mut end = 0;
    let mut hs: HashSet<u8> = HashSet::new();

    while end < s.len() {
        while hs.contains(&s[end]) {
            hs.remove(&s[start]);
            start += 1;
        }
        hs.insert(s[end]);
        end += 1;
        if end - start == marker_length {
            break;
        }
    }
    return end as u32;
}

fn part_1(s: &str) -> u32 {
    solve(s, 4)
}

fn part_2(s: &str) -> u32 {
    solve(s, 14)
}

fn main() {
    let data = std::fs::read_to_string("input/06.txt").expect("Unable to read file");
    let part_1 = part_1(&data);
    let part_2 = part_2(&data);
    println!("Part 1: {}. Part 2: {}", part_1, part_2);
}

mod tests {
    use super::*;

    #[test]
    fn test() {
        assert!(part_1("mjqjpqmgbljsphdztnvjfqwrcgsmlb") == 7);
        assert!(part_1("bvwbjplbgvbhsrlpgdmjqwftvncz") == 5);
        assert!(part_1("nppdvjthqldpwncqszvftbrmjlhg") == 6);
        assert!(part_1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg") == 10);
        assert!(part_1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw") == 11);
        assert!(part_2("mjqjpqmgbljsphdztnvjfqwrcgsmlb") == 19);
        assert!(part_2("bvwbjplbgvbhsrlpgdmjqwftvncz") == 23);
        assert!(part_2("nppdvjthqldpwncqszvftbrmjlhg") == 23);
        assert!(part_2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg") == 29);
        assert!(part_2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw") == 26);
    }
}
