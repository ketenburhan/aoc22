use std::collections::HashSet;

const DATA: &str = include_str!("data.txt");

fn get_priority(c: char) -> usize {
    if c.is_lowercase() {
        c as usize - 'a' as usize + 1
    } else {
        c as usize - 'A' as usize + 27
    }
}

fn get_common(strings: &[&str]) -> char {
    let mut out: HashSet<char> = strings[0].chars().collect();
    for item in strings.iter().skip(1) {
        let hs: HashSet<char> = item.chars().collect();
        out = out.intersection(&hs).cloned().collect();
    }
    out.into_iter().next().unwrap()
}

fn main() {
    println!("part 1: {:?}", part1(DATA));
    println!("part 2: {:?}", part2(DATA));
}

fn part1(data: &str) -> usize {
    data.lines()
        .map(|line| {
            let half = line.len() / 2;
            let left_comp = &line[..half];
            let right_comp = &line[half..];

            get_priority(get_common(&[left_comp, right_comp]))
        })
        .sum()
}

fn part2(data: &str) -> usize {
    data.lines()
        .collect::<Vec<&str>>()
        .chunks(3)
        .map(|lines| get_priority(get_common(lines)))
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    const DATA: &str = include_str!("example.txt");

    #[test]
    fn part1_test() {
        assert_eq!(part1(DATA), 157);
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2(DATA), 70);
    }
}
