const DATA: &str = include_str!("data.txt");

type State = Vec<Vec<char>>;

fn main() {
    println!("part 1: {:?}", part1(DATA));
    println!("part 2: {:?}", part2(DATA));
}

fn part1(data: &str) -> String {
    let (initial, instructions) = data.split_once("\n\n").unwrap();
    let mut state = get_initial_state(initial);

    for ins in instructions.lines() {
        let mut ins_split = ins.split_ascii_whitespace();
        ins_split.next();
        let count: usize = ins_split.next().unwrap().parse().unwrap();
        ins_split.next();
        let from: usize = ins_split.next().unwrap().parse().unwrap();
        ins_split.next();
        let to: usize = ins_split.next().unwrap().parse().unwrap();

        for _i in 0..count {
            let val = state[from - 1].pop().unwrap();
            state[to - 1].push(val);
        }
    }

    state.iter_mut().map(|stack| stack.pop().unwrap()).collect()
}

fn part2(data: &str) -> String {
    let (initial, instructions) = data.split_once("\n\n").unwrap();
    let mut state = get_initial_state(initial);

    for ins in instructions.lines() {
        let mut ins_split = ins.split_ascii_whitespace();
        ins_split.next();
        let count: usize = ins_split.next().unwrap().parse().unwrap();
        ins_split.next();
        let from: usize = ins_split.next().unwrap().parse().unwrap();
        ins_split.next();
        let to: usize = ins_split.next().unwrap().parse().unwrap();

        let len = state[from - 1].len();
        let mut vals: Vec<char> = state[from - 1].drain(len - count..).collect();
        state[to - 1].append(&mut vals);
    }

    state.iter_mut().map(|stack| stack.pop().unwrap()).collect()
}

fn get_initial_state(s: &str) -> State {
    let stack_count = (s.find('\n').unwrap() + 1) / 4;
    let mut out = vec![vec![]; stack_count];
    let rev_lines = s.lines().rev().skip(1);

    for line in rev_lines {
        for (i, stack) in out.iter_mut().enumerate() {
            let val = line.chars().nth(i * 4 + 1).unwrap();
            if val != ' ' {
                stack.push(val);
            }
        }
    }

    println!("{:?}", out);
    out
}

#[cfg(test)]
mod test {
    use super::*;

    const DATA: &str = include_str!("example.txt");

    #[test]
    fn part1_test() {
        assert_eq!(&part1(DATA), "CMZ");
    }

    #[test]
    fn part2_test() {
        assert_eq!(&part2(DATA), "MCD");
    }
}
