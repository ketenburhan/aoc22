const DATA: &str = include_str!("data.txt");

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Move {
    Rock = 0,
    Paper,
    Scissors,
}

impl Move {
    fn from_usize(num: usize) -> Self {
        match num {
            0 => Self::Rock,
            1 => Self::Paper,
            2 => Self::Scissors,
            _ => unreachable!("{}", num),
        }
    }
    fn from_abc(s: &str) -> Self {
        match s {
            "A" => Self::Rock,
            "B" => Self::Paper,
            "C" => Self::Scissors,
            _ => unreachable!(),
        }
    }
    fn get_winning_against(self) -> Self {
        Self::from_usize((self as usize + 2) % 3)
    }
    fn get_losing_against(self) -> Self {
        Self::from_usize((self as usize + 1) % 3)
    }
    fn get_draw_against(self) -> Self {
        self
    }
    fn get_outcome_against(self, opponent: Self) -> usize {
        if self == opponent {
            return 3;
        } else if self.get_winning_against() == opponent {
            return 6;
        } else {
            return 0;
        }
    }
    fn get_score_for_shape(self) -> usize {
        self as usize + 1
    }
}

fn main() {
    println!("part 1: {:?}", part1(DATA));
    println!("part 2: {:?}", part2(DATA));
}

fn part1(data: &str) -> usize {
    data.lines()
        .map(|line| {
            let mut chars = line.chars();
            let oppo_move = Move::from_abc(&chars.next().unwrap().to_string());
            chars.next();
            let my_move = match chars.next().unwrap().to_string().as_str() {
                "X" => Move::Rock,
                "Y" => Move::Paper,
                "Z" => Move::Scissors,
                _ => unreachable!(),
            };

            let score_for_shape = my_move.get_score_for_shape();

            let outcome = my_move.get_outcome_against(oppo_move);

            outcome + score_for_shape
        })
        .sum()
}

fn part2(data: &str) -> usize {
    data.lines()
        .map(|line| {
            let mut chars = line.chars();
            let oppo_move = Move::from_abc(&chars.next().unwrap().to_string());
            chars.next();
            let mut outcome = 0;
            let my_move = match chars.next().unwrap().to_string().as_str() {
                "X" => oppo_move.get_winning_against(),
                "Y" => {
                    outcome = 3;
                    oppo_move.get_draw_against()
                }
                "Z" => {
                    outcome = 6;
                    oppo_move.get_losing_against()
                }
                _ => unreachable!(),
            };

            let score_for_shape = my_move.get_score_for_shape();

            outcome + score_for_shape
        })
        .sum()
}
#[cfg(test)]
mod test {
    use super::*;

    const DATA: &str = include_str!("example.txt");

    #[test]
    fn part1_test() {
        assert_eq!(part1(DATA), 15);
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2(DATA), 12);
    }
}
