use std::{collections::HashMap, path::PathBuf, rc::Rc};

const DATA: &str = include_str!("data.txt");

fn main() {
    println!("part 1: {:?}", part1(DATA));
    println!("part 2: {:?}", part2(DATA));
}

#[derive(Debug, Clone)]
enum FSItem {
    File(String, usize),
    Directory(String, Option<Vec<Rc<FSItem>>>),
}

impl From<&str> for FSItem {
    fn from(s: &str) -> Self {
        let (part1, name) = s.split_once(' ').unwrap();
        match part1 {
            "dir" => Self::Directory(name.to_string(), None),
            n_str => Self::File(name.to_string(), n_str.parse().unwrap()),
        }
    }
}

fn get_indirect_size(item_direct_sizes: &HashMap<PathBuf, usize>, path: &PathBuf) -> usize {
    item_direct_sizes
        .iter()
        .map(|(sub_path, sub_direct_size)| {
            if sub_path.starts_with(path) {
                *sub_direct_size
            } else {
                0
            }
        })
        .sum()
}

fn get_item_direct_sizes(data: &str) -> HashMap<PathBuf, usize> {
    let mut path = PathBuf::from("/");
    let mut item_direct_sizes: HashMap<PathBuf, usize> = HashMap::new();
    let mut command_in_out = data.split('$');

    // skip first empty string
    command_in_out.next();

    command_in_out
        .map(|s| {
            let (cmd, out) = s.split_once('\n').unwrap();
            (cmd.trim(), out.trim())
        })
        .for_each(|(cmd, out)| {
            match cmd {
                "ls" => {
                    let size: usize = out
                        .lines()
                        .map(FSItem::from)
                        .filter_map(|item| match item {
                            FSItem::File(_, size) => Some(size),
                            _ => None,
                        })
                        .sum();
                    item_direct_sizes.insert(path.clone(), size);
                }
                // cd
                _ => match cmd.split_once(' ').unwrap().1 {
                    "/" => path = PathBuf::from("/"),
                    ".." => {
                        path.pop();
                    }
                    d => path.push(d),
                },
            }
        });
    item_direct_sizes
}

fn part1(data: &str) -> usize {
    let item_direct_sizes: HashMap<PathBuf, usize> = get_item_direct_sizes(data);

    item_direct_sizes
        .iter()
        .map(|(path, _)| {
            let size = get_indirect_size(&item_direct_sizes, path);
            if size <= 100_000 {
                size
            } else {
                0
            }
        })
        .sum()
}

const TOTAL: usize = 70_000_000;
const UNUSED_NEEDED: usize = 30_000_000;
const MAX_USED_NEEDED: usize = TOTAL - UNUSED_NEEDED;

fn part2(data: &str) -> usize {
    let item_direct_sizes: HashMap<PathBuf, usize> = get_item_direct_sizes(data);
    let total_used = get_indirect_size(&item_direct_sizes, &PathBuf::from("/"));
    let min_free_up_space = total_used - MAX_USED_NEEDED;

    let mut filtered = item_direct_sizes
        .iter()
        .map(|(path, _)| get_indirect_size(&item_direct_sizes, path))
        .filter(|size| *size >= min_free_up_space)
        .collect::<Vec<_>>();
    filtered.sort();
    filtered[0]
}

#[cfg(test)]
mod test {
    use super::*;

    const DATA: &str = include_str!("example.txt");

    #[test]
    fn part1_test() {
        assert_eq!(part1(DATA), 95437);
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2(DATA), 24933642);
    }
}
