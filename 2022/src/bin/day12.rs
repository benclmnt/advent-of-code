use std::collections::binary_heap;

fn main() {
    let data = std::fs::read_to_string("input/12.txt").expect("Unable to read file");
    let data = process_in(&data);
    let part_1 = part_1(&data);
    let part_2 = part_2(&data);
    println!("Part 1: {}. Part 2: {}", part_1, part_2);
}

struct Data {
    terrain: Vec<Vec<char>>,
    start: (i16, i16),
    end: (i16, i16),
}

fn part_1(data: &Data) -> i32 {
    let mut visited = vec![vec![false; data.terrain[0].len()]; data.terrain.len()];
    let mut heap = binary_heap::BinaryHeap::new();
    let curr = (0, 'z', data.end.0, data.end.1);
    heap.push(curr);
    loop {
        let curr = heap.pop().unwrap();
        if curr.2 == data.start.0 && curr.3 == data.start.1 {
            break -curr.0;
        }
        if visited[curr.2 as usize][curr.3 as usize] {
            continue;
        }
        visited[curr.2 as usize][curr.3 as usize] = true;
        for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            if curr.2 + dx >= 0
                && curr.2 + dx < data.terrain.len() as i16
                && curr.3 + dy >= 0
                && curr.3 + dy < data.terrain[0].len() as i16
                && curr.1 as i8 - data.terrain[(curr.2 + dx) as usize][(curr.3 + dy) as usize] as i8
                    <= 1
            {
                heap.push((
                    curr.0 - 1,
                    data.terrain[(curr.2 + dx) as usize][(curr.3 + dy) as usize],
                    curr.2 + dx,
                    curr.3 + dy,
                ));
            }
        }
    }
}

fn part_2(data: &Data) -> i32 {
    let mut visited = vec![vec![false; data.terrain[0].len()]; data.terrain.len()];
    let mut heap = binary_heap::BinaryHeap::new();
    let curr = (0, 'z', data.end.0, data.end.1);
    heap.push(curr);
    loop {
        let curr = heap.pop().unwrap();
        // only changed here from part 1
        if curr.1 == 'a' {
            break -curr.0;
        }
        if visited[curr.2 as usize][curr.3 as usize] {
            continue;
        }
        visited[curr.2 as usize][curr.3 as usize] = true;
        for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            if curr.2 + dx >= 0
                && curr.2 + dx < data.terrain.len() as i16
                && curr.3 + dy >= 0
                && curr.3 + dy < data.terrain[0].len() as i16
                && curr.1 as i8 - data.terrain[(curr.2 + dx) as usize][(curr.3 + dy) as usize] as i8
                    <= 1
            {
                heap.push((
                    curr.0 - 1,
                    data.terrain[(curr.2 + dx) as usize][(curr.3 + dy) as usize],
                    curr.2 + dx,
                    curr.3 + dy,
                ));
            }
        }
    }
}

fn process_in(s: &str) -> Data {
    let mut terrain = vec![];
    let mut start = (0, 0);
    let mut end = (0, 0);
    for (i, line) in s.lines().enumerate() {
        let mut row = Vec::with_capacity(line.len());
        for (j, c) in line.chars().enumerate() {
            match c {
                'S' => {
                    start = (i as i16, j as i16);
                    row.push('a');
                }
                'E' => {
                    end = (i as i16, j as i16);
                    row.push('z');
                }
                _ => row.push(c),
            }
        }
        terrain.push(row);
    }
    Data {
        terrain,
        start,
        end,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";
    #[test]
    fn test() {
        let terrain = process_in(&INPUT);
        assert!(part_1(&terrain) == 31);
        assert!(part_2(&terrain) == 29);
    }
}
