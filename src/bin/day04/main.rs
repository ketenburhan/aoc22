use std::collections::HashSet;

use regex::Regex;

const DATA: &str = include_str!("data.txt");

fn main() {
    println!("part 1: {:?}", part1(DATA));
    println!("part 2: {:?}", part2(DATA));
}

fn get_assignment_pair(pair: &str) -> (HashSet<usize>, HashSet<usize>) {
    let re = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();
    let caps = re.captures(pair).unwrap();
    let mut caps = caps
        .iter()
        .skip(1)
        .take(4)
        .map(|s| s.unwrap().as_str().parse::<usize>().unwrap());
    (
        (caps.next().unwrap()..=caps.next().unwrap()).collect(),
        (caps.next().unwrap()..=caps.next().unwrap()).collect(),
    )
}

fn part1(data: &str) -> usize {
    data.lines()
        .filter(|line| {
            let (a0, a1) = get_assignment_pair(line);
            a0.is_subset(&a1) || a0.is_superset(&a1)
        })
        .count()
}

fn part2(data: &str) -> usize {
    data.lines()
        .filter(|line| {
            let (a0, a1) = get_assignment_pair(line);
            a0.intersection(&a1).next().is_some()
        })
        .count()
}

#[cfg(test)]
mod test {
    use super::*;

    const DATA: &str = include_str!("example.txt");

    #[test]
    fn part1_test() {
        assert_eq!(part1(DATA), 2);
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2(DATA), 4);
    }
}
