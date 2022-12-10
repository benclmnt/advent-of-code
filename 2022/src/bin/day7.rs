// Each Dir or File contains the index of the corresponding Directory or File in Data
#[derive(PartialEq, Copy, Clone, Debug)]
enum DirOrFile {
    Dir(u8),
    File(u8),
}

#[derive(Debug)]
struct Directory {
    name: String,
    /// total size of all directory or files in it.
    size: u32,
    children: Vec<DirOrFile>,
    /// None only if root
    parent_dir: Option<DirOrFile>,
    id: DirOrFile,
}

#[derive(Debug)]
#[allow(dead_code)]
struct File {
    name: String,
    size: u32,
    parent_dir: DirOrFile,
    id: DirOrFile,
}

#[derive(Debug)]
struct Data {
    dir_list: Vec<Directory>,
    file_list: Vec<File>,
}

fn process_in(s: &str) -> Data {
    let mut data = Data {
        dir_list: vec![Directory {
            name: String::from("/"),
            size: 0,
            children: vec![],
            parent_dir: None,
            id: DirOrFile::Dir(0),
        }],
        file_list: vec![],
    };

    let mut dir_stack = vec![];
    for line in s.lines() {
        let tokens = line.split(' ').collect::<Vec<_>>();
        if tokens[0] != "$" {
            if tokens[0] == "dir" {
                // directory
                let dir_name = tokens[1];
                let dir_id = DirOrFile::Dir(data.dir_list.len() as u8);
                let parent_id = *dir_stack.last().unwrap();
                if let DirOrFile::Dir(parent_id) = parent_id {
                    // add this directory as children of the parent directory
                    data.dir_list[parent_id as usize].children.push(dir_id);
                }
                // add this directory to the dir_list
                data.dir_list.push(Directory {
                    name: dir_name.to_string(),
                    size: 0,
                    children: vec![],
                    parent_dir: Some(parent_id),
                    id: dir_id,
                });
            } else {
                // file
                let file_size = tokens[0].parse::<u32>().unwrap();
                let file_name = tokens[1];
                let parent_id = *dir_stack.last().unwrap();
                let file_id = DirOrFile::File(data.file_list.len() as u8);
                if let DirOrFile::Dir(mut parent_id) = parent_id {
                    // add this directory as children of the parent directory
                    data.dir_list[parent_id as usize].children.push(file_id);
                    // update directory sizes recursively
                    loop {
                        data.dir_list[parent_id as usize].size += file_size;
                        parent_id = if let Some(DirOrFile::Dir(x)) =
                            data.dir_list[parent_id as usize].parent_dir
                        {
                            x
                        } else {
                            break;
                        };
                    }
                }
                // add this directory to the dir_list
                data.file_list.push(File {
                    name: file_name.to_string(),
                    size: file_size,
                    parent_dir: parent_id,
                    id: file_id,
                });
            }
        } else {
            // command
            if tokens[1] == "ls" {
                // nothing to do
            } else if tokens[1] == "cd" {
                if tokens[2] == ".." {
                    dir_stack.pop();
                } else if tokens[2] == "/" {
                    dir_stack = vec![DirOrFile::Dir(0)];
                } else {
                    let dir_name = tokens[2];
                    let dir_index = *dir_stack.last().unwrap();
                    dir_stack.push(
                        data.dir_list
                            .iter()
                            .find(|d| d.name == dir_name && d.parent_dir == Some(dir_index))
                            .unwrap()
                            .id,
                    );
                }
            }
        }
    }
    data
}

/// Return the sum of all directories with total size of at most 100000
fn part_1(data: &Data) -> u32 {
    data.dir_list
        .iter()
        .map(|d| d.size)
        .filter(|x| *x <= 100_000)
        .sum()
}

/// Return the smallest directory that we need to delete so that we can run update.
fn part_2(data: &Data) -> u32 {
    const DISK_SPACE: u32 = 70_000_000;
    const DESIRED_SPACE: u32 = 30_000_000;
    let unused_space: u32 = DISK_SPACE - data.dir_list[0].size;
    data.dir_list
        .iter()
        .map(|d| d.size)
        .filter(|x| unused_space + *x >= DESIRED_SPACE)
        .min()
        .unwrap()
}

fn main() {
    let terminal_output = std::fs::read_to_string("input/07.txt").expect("Unable to read file");
    let data = process_in(&terminal_output);
    let part_1 = part_1(&data);
    let part_2 = part_2(&data);
    println!("Part 1: {}. Part 2: {}", part_1, part_2);
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    #[test]
    fn test() {
        let data = process_in(&INPUT);
        assert!(part_1(&data) == 95437);
        assert!(part_2(&data) == 24933642);
    }
}
