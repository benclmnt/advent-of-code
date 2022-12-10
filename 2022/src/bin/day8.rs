type Data = Vec<Vec<i8>>;

fn process_in(s: &str) -> Data {
    s.lines()
        .map(|line| {
            line.chars()
                .map(|x| x as i8 - '0' as i8)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn part_1_logic(
    data: &Data,
    bitmap: &mut [Vec<bool>],
    count: &mut u32,
    i: usize,
    j: usize,
    current_max_height: &mut i8,
) {
    if data[i][j] > *current_max_height {
        *current_max_height = data[i][j];
        *count += !bitmap[i][j] as u32;
        bitmap[i][j] = true;
    }
}

fn part_1(data: &Data) -> u32 {
    let mut bitmap = vec![vec![false; data[0].len()]; data.len()];
    let mut count = 0;
    for i in 0..data.len() {
        // l -> r
        let mut current_max_height = -1;
        for j in 0..data[i].len() {
            part_1_logic(data, &mut bitmap, &mut count, i, j, &mut current_max_height);
        }

        // r -> l
        let mut current_max_height = -1;
        for j in (0..data[i].len()).rev() {
            part_1_logic(data, &mut bitmap, &mut count, i, j, &mut current_max_height);
        }
    }

    for j in 0..data[0].len() {
        // t -> b
        let mut current_max_height = -1;
        for i in 0..data.len() {
            part_1_logic(data, &mut bitmap, &mut count, i, j, &mut current_max_height);
        }
        // b -> t
        let mut current_max_height = -1;
        for i in (0..data.len()).rev() {
            part_1_logic(data, &mut bitmap, &mut count, i, j, &mut current_max_height);
        }
    }

    count
}

fn part_2(data: &Data) -> u32 {
    let mut scores = vec![vec![1; data[0].len()]; data.len()];
    let mut max_score = 0;
    for i in 0..data.len() {
        // l -> r
        let mut stack: Vec<(i8, usize)> = vec![(std::i8::MAX, 0)];
        for j in 0..data[i].len() {
            let idx = stack.partition_point(|x| x.0 >= data[i][j]);
            scores[i][j] *= j - stack[idx - 1].1;
            stack.resize(idx, (0, 0));
            stack.push((data[i][j], j));
        }

        // r -> l
        let mut stack: Vec<(i8, usize)> = vec![(std::i8::MAX, data[i].len() - 1)];
        for j in (0..data[i].len()).rev() {
            let idx = stack.partition_point(|x| x.0 >= data[i][j]);
            scores[i][j] *= stack[idx - 1].1 - j;
            stack.resize(idx, (0, 0));
            stack.push((data[i][j], j));
        }
    }

    for j in 0..data[0].len() {
        // t -> b
        let mut stack: Vec<(i8, usize)> = vec![(std::i8::MAX, 0)];
        for i in 0..data.len() {
            let idx = stack.partition_point(|x| x.0 >= data[i][j]);
            scores[i][j] *= i - stack[idx - 1].1;
            stack.resize(idx, (0, 0));
            stack.push((data[i][j], i));
        }

        // b -> t
        let mut stack: Vec<(i8, usize)> = vec![(std::i8::MAX, data[0].len() - 1)];
        for i in (0..data.len()).rev() {
            let idx = stack.partition_point(|x| x.0 >= data[i][j]);
            scores[i][j] *= stack[idx - 1].1 - i;
            max_score = std::cmp::max(max_score, scores[i][j]);
            stack.resize(idx, (0, 0));
            stack.push((data[i][j], i));
        }
    }

    max_score as u32
}

fn main() {
    let terminal_output = std::fs::read_to_string("input/08.txt").expect("Unable to read file");
    let data = process_in(&terminal_output);
    let part_1 = part_1(&data);
    let part_2 = part_2(&data);
    println!("Part 1: {}. Part 2: {}", part_1, part_2);
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "30373
25512
65332
33549
35390";

    #[test]
    fn test() {
        let data = process_in(&INPUT);
        assert!(part_1(&data) == 21);
        assert!(part_2(&data) == 8);
    }
}
