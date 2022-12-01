const DATA: &str = include_str!("data.txt");

fn main() {
    println!("part 1: {:?}", part1());
    println!("part 2: {:?}", part2());
}

fn part1() -> usize {
    get_elves_cals().max().unwrap()
}

fn part2() -> usize {
    let mut elves_cals: Vec<usize> = get_elves_cals().collect();
    elves_cals.sort_by(|a, b| b.cmp(a));
    elves_cals.iter().take(3).sum()
}

fn get_elves_cals() -> impl Iterator<Item = usize> {
    DATA.split("\n\n").map(|elf| {
        elf.lines()
            .map(|line| line.parse::<usize>().unwrap())
            .sum::<usize>()
    })
}
