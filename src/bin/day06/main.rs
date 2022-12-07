use std::collections::HashSet;

const DATA: &str = include_str!("data.txt");

fn main() {
    println!("part 1: {:?}", part1(DATA));
    println!("part 2: {:?}", part2(DATA));
}

fn part1(data: &str) -> usize {
    let pos = data
        .chars()
        .collect::<Vec<char>>()
        .windows(4)
        .position(|win| {
            let set: HashSet<char> = win.iter().cloned().collect();
            set.len() == 4
        });

    pos.unwrap() + 4
}

fn part2(data: &str) -> usize {
    let pos = data
        .chars()
        .collect::<Vec<char>>()
        .windows(14)
        .position(|win| {
            let set: HashSet<char> = win.iter().cloned().collect();
            set.len() == 14
        });

    pos.unwrap() + 14
}

#[cfg(test)]
mod test {
    use super::*;

    const DATA: &str = include_str!("example.txt");

    #[test]
    fn part1_test() {
        assert_eq!(part1(DATA), 7);
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2(DATA), 19);
    }
}
