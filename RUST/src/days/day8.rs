use std::fs::{self};

fn count_visible_trees_refactored(grid: &Vec<Vec<u8>>, height: &usize, width: &usize) -> usize {
    let mut visible_trees = 0;

    for y in 1..(height - 1) {
        for x in 1..(width - 1) {
            let height_tree = grid[y][x];

            if grid[y][0..x].iter().any(|tree| *tree >= height_tree)
                && grid[y][(x + 1)..].iter().any(|tree| *tree >= height_tree)
                && grid[0..y].iter().any(|row| row[x] >= height_tree)
                && grid[(y + 1)..].iter().any(|row| row[x] >= height_tree)
            {
                visible_trees += 1;
            }
        }
    }

    width * height - visible_trees
}

fn scenic_score_refactored(grid: &Vec<Vec<u8>>, height: &usize, width: &usize) -> usize {
    let mut max_scenic_score = 0;
    for y in 1..(height - 1) {
        for x in 1..(width - 1) {
            let height_tree = grid[y][x];

            let mut scenic_score = 1;

            // grid[y][0..x].iter().rev() => going from the tree to the edge
            scenic_score *= if let Some((pos, _)) = grid[y][0..x]
                .iter()
                .rev()
                .enumerate()
                .find(|(_, tree)| **tree >= height_tree)
            {
                pos + 1
            } else {
                x
            };

            scenic_score *= if let Some((pos, _)) = grid[y][(x + 1)..]
                .iter()
                .enumerate()
                .find(|(_, tree)| **tree >= height_tree)
            {
                pos + 1
            } else {
                width - x - 1
            };

            scenic_score *= if let Some((pos, _)) = grid[0..y]
                .iter()
                .rev()
                .enumerate()
                .find(|(_, row)| row[x] >= height_tree)
            {
                pos + 1
            } else {
                y
            };

            scenic_score *= if let Some((pos, _)) = grid[(y + 1)..]
                .iter()
                .enumerate()
                .find(|(_, row)| row[x] >= height_tree)
            {
                pos + 1
            } else {
                height - y - 1
            };

            if scenic_score > max_scenic_score {
                max_scenic_score = scenic_score;
            }
        }
    }

    max_scenic_score
}

pub fn run() {
    let input = fs::read_to_string("src/inputs/day8").unwrap();

    let width = input.lines().last().unwrap().len();
    let height = input.lines().count();

    let grid: Vec<Vec<u8>> = input
        .lines()
        .map(|line| line.chars().map(|h| (h as u8 - b'0') as u8).collect())
        .collect();

    let visible_trees = count_visible_trees_refactored(&grid, &height, &width);
    let scenic_score = scenic_score_refactored(&grid, &height, &width);

    println!("Visible trees: {}", visible_trees);
    println!("Scenic score: {}", scenic_score);
}
