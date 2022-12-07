use std::{collections::HashMap, str::FromStr};

struct Dir {
    dirs: Vec<String>,
    files_size: u32,
}

fn parse_input(input: &str) -> HashMap<String, u32> {
    let mut input = input.lines().peekable();

    let mut cwd = Vec::new();
    let mut directories = HashMap::new();
    'outer: loop {
        loop {
            let Some(command) = input.next() else {break 'outer;};
            if command == "$ ls" {
                break;
            }
            // assume command is a `cd`
            let dir_name = String::from_utf8(command.as_bytes()[5..].into()).unwrap();
            if dir_name == ".." {
                cwd.pop().expect("cannot move out");
            } else {
                cwd.push(dir_name);
            }
        }
        let path = cwd.join("/");
        let mut dirs = Vec::new();
        let mut files_size = 0;
        loop {
            match input.peek() {
                Some(cmd) if cmd.starts_with('$') => break,
                Some(&file) => {
                    let _ = input.next(); // consume the line
                    let (size, name) = file.split_once(' ').unwrap();
                    if size == "dir" {
                        let dir_path = format!("{path}/{name}");
                        dirs.push(dir_path);
                    } else {
                        let size: u32 = FromStr::from_str(size).unwrap();
                        files_size += size;
                    }
                }
                None => break,
            }
        }
        directories.insert(path, Dir { dirs, files_size });
    }

    fn get_dir_size(
        dir_sizes: &mut HashMap<String, u32>,
        directories: &mut HashMap<String, Dir>,
        path: String,
    ) -> u32 {
        let dir = directories.remove(&path).expect("invalid dir path");
        let dirs_size: u32 = dir
            .dirs
            .into_iter()
            .map(|path| get_dir_size(dir_sizes, directories, path))
            .sum();
        let size = dir.files_size + dirs_size;
        dir_sizes.insert(path, size);
        size
    }

    let mut dir_sizes = HashMap::new();
    let _ = get_dir_size(&mut dir_sizes, &mut directories, "/".to_string());
    dir_sizes
}

pub fn part_one(input: &str) -> Option<u32> {
    let directories = parse_input(input);

    let sum_of_size = directories.values().filter(|size| size <= &&100000).sum();

    Some(sum_of_size)
}

pub fn part_two(input: &str) -> Option<u32> {
    let directories = parse_input(input);

    const TOTAL_SPACE: u32 = 70000000;
    const REQUIRED_SPACE: u32 = 30000000;
    let used_space = directories.get("/").unwrap();
    let free_space = TOTAL_SPACE - used_space;
    let to_free = REQUIRED_SPACE - free_space;

    let to_remove_size = directories
        .values()
        .copied()
        .filter(|size| size >= &to_free)
        .min()
        .unwrap();

    Some(to_remove_size)
}

fn main() {
    let input = &adventofcode::read_file("inputs", 7);
    {
        // HashMap does some stuff when used for the first time
        // that took ~300 µs (on my machine).
        // Create a HashSet here to not impact the solutions performance.
        let mut dummy = HashMap::with_capacity(1);
        dummy.insert(0u8, 0u8);
    }
    adventofcode::solve!(1, part_one, input);
    adventofcode::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = adventofcode::read_file("examples", 7);
        assert_eq!(part_one(&input), Some(95437));
    }

    #[test]
    fn test_part_two() {
        let input = adventofcode::read_file("examples", 7);
        assert_eq!(part_two(&input), Some(24933642));
    }
}
