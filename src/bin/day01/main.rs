const DATA: &str = include_str!("data.txt");

fn main() {
    println!("part 1: {:?}", part1(DATA));
    println!("part 2: {:?}", part2(DATA));
}

fn part1(data: &str) -> usize {
    get_elves_cals(data).max().unwrap()
}

fn part2(data: &str) -> usize {
    let mut elves_cals: Vec<usize> = get_elves_cals(data).collect();
    elves_cals.sort_by(|a, b| b.cmp(a));
    elves_cals.iter().take(3).sum()
}

fn get_elves_cals(data: &str) -> impl Iterator<Item = usize> + '_ {
    data.split("\n\n").map(|elf| {
        elf.lines()
            .map(|line| line.parse::<usize>().unwrap())
            .sum::<usize>()
    })
}

#[cfg(test)]
mod test {
    use super::*;

    const DATA: &str = include_str!("example.txt");

    #[test]
    fn part1_test() {
        assert_eq!(part1(DATA), 24000);
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2(DATA), 45000);
    }
}
