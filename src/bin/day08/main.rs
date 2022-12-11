const DATA: &str = include_str!("data.txt");

fn main() {
    println!("part 1: {:?}", part1(DATA));
    println!("part 2: {:?}", part2(DATA));
}

type Grid = Vec<Vec<String>>;

fn get_len<T>(trees_grid: &Grid, x: T, y: T) -> u8
where
    T: Into<usize>,
{
    trees_grid[y.into()][x.into()].parse::<u8>().unwrap()
}

fn part1(data: &str) -> usize {
    let trees_grid: Grid = data
        .lines()
        .map(|line| line.chars().map(|c| c.to_string()).collect())
        .collect();
    let width = trees_grid[0].len();
    let height = trees_grid.len();
    let trees_on_edge_count = (width + height) * 2 - 4;
    let mut count = trees_on_edge_count;

    for y in 1..height - 1 {
        for x in 1..width - 1 {
            let tree_len = get_len(&trees_grid, x, y);

            if (0..x).all(|left| tree_len > get_len(&trees_grid, left, y))
                || (0..y).all(|top| tree_len > get_len(&trees_grid, x, top))
                || (x + 1..width).all(|right| tree_len > get_len(&trees_grid, right, y))
                || (y + 1..height).all(|bottom| tree_len > get_len(&trees_grid, x, bottom))
            {
                count += 1;
            }
        }
    }

    count
}

fn get_visible_trees_horizontal(
    trees_grid: &Grid,
    tree_len: u8,
    range: impl Iterator<Item = usize>,
    row: usize,
) -> usize {
    let mut count = 0;
    for i in range {
        let other_tree_len = get_len(trees_grid, i, row);
        count += 1;
        if tree_len <= other_tree_len {
            break;
        }
    }
    count
}

fn get_visible_trees_vertical(
    trees_grid: &Grid,
    tree_len: u8,
    range: impl Iterator<Item = usize>,
    column: usize,
) -> usize {
    let mut count = 0;
    for i in range {
        let other_tree_len = get_len(trees_grid, column, i);
        count += 1;
        if tree_len <= other_tree_len {
            break;
        }
    }
    count
}

fn part2(data: &str) -> usize {
    let trees_grid: Grid = data
        .lines()
        .map(|line| line.chars().map(|c| c.to_string()).collect())
        .collect();
    let width = trees_grid[0].len();
    let height = trees_grid.len();
    let mut max_scenic_score: usize = 0;

    for y in 1..height - 1 {
        for x in 1..width - 1 {
            let tree_len = get_len(&trees_grid, x, y);

            let left_tree_count =
                get_visible_trees_horizontal(&trees_grid, tree_len, (0..=x - 1).rev(), y);

            let top_tree_count =
                get_visible_trees_vertical(&trees_grid, tree_len, (0..=y - 1).rev(), x);

            let right_tree_count =
                get_visible_trees_horizontal(&trees_grid, tree_len, x + 1..width, y);

            let bottom_tree_count =
                get_visible_trees_vertical(&trees_grid, tree_len, y + 1..height, x);

            let scenic_score =
                left_tree_count * top_tree_count * right_tree_count * bottom_tree_count;

            if scenic_score > max_scenic_score {
                max_scenic_score = scenic_score;
            }
        }
    }

    max_scenic_score
}

#[cfg(test)]
mod test {
    use super::*;

    const DATA: &str = include_str!("example.txt");

    #[test]
    fn part1_test() {
        assert_eq!(part1(DATA), 21);
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2(DATA), 8);
    }
}
