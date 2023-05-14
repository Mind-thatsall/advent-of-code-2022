use std::fs;

fn count_visible_trees(grid: &[Vec<u32>]) -> usize {
    let max_length = grid.len() - 1;
    let mut visible_trees: Vec<Vec<bool>> = grid
        .iter()
        .enumerate()
        .map(|(idx, grid_line)| {
            let line_idx_max_length = grid_line.len() - 1;
            grid_line
                .iter()
                .enumerate()
                .map(|(line_idx, _)| {
                    idx == 0
                        || idx == max_length
                        || line_idx == 0
                        || line_idx == line_idx_max_length
                })
                .collect()
        })
        .collect();

    for y in 0..grid.len() {
        let mut current_tree_size = 0;
        for x in 0..grid[0].len() {
            if x == 0 {
                current_tree_size = grid[y][x];
            } else if grid[y][x] > current_tree_size {
                current_tree_size = grid[y][x];
                visible_trees[y][x] = true;
            }
        }
    }

    for y in 0..grid.len() {
        let mut current_tree_size = 0;
        for x in (0..grid[0].len()).rev() {
            if x == grid.len() - 1 {
                current_tree_size = grid[y][x];
            } else if grid[y][x] > current_tree_size {
                current_tree_size = grid[y][x];
                visible_trees[y][x] = true;
            }
        }
    }

    for x in 0..grid.len() {
        let mut current_tree_size = 0;
        for y in 0..grid[0].len() {
            if y == 0 {
                current_tree_size = grid[y][x];
            } else if grid[y][x] > current_tree_size {
                current_tree_size = grid[y][x];
                visible_trees[y][x] = true;
            }
        }
    }

    for x in 0..grid.len() {
        let mut current_tree_size = 0;
        for y in (0..grid[0].len()).rev() {
            if y == grid.len() - 1 {
                current_tree_size = grid[y][x];
            } else if grid[y][x] > current_tree_size {
                current_tree_size = grid[y][x];
                visible_trees[y][x] = true;
            }
        }
    }

    visible_trees.iter().flatten().filter(|&&val| val).count()
}

fn best_scenic_view(grid: &[Vec<u32>]) -> usize {
    let mut highest_score = 0;

    let y_max = grid.len();
    let x_max = grid[0].len();
    for (y_idx, grid_line) in grid.iter().enumerate() {
        for (x_idx, grid_height) in grid_line.iter().enumerate() {
            let mut scores = [0, 0, 0, 0];

            for x_pos in (0..x_idx).rev() {
                if grid[y_idx][x_pos] < *grid_height {
                    scores[0] += 1;
                } else {
                    scores[0] += 1;
                    break;
                }
            }

            for x_pos in (x_idx + 1)..x_max {
                if grid[y_idx][x_pos] < *grid_height {
                    scores[1] += 1;
                } else {
                    scores[1] += 1;
                    break;
                }
            }

            for y_pos in (0..y_idx).rev() {
                if grid[y_pos][x_idx] < *grid_height {
                    scores[2] += 1;
                } else {
                    scores[2] += 1;
                    break;
                }
            }

            for y_pos in (y_idx + 1)..y_max {
                if grid[y_pos][x_idx] < *grid_height {
                    scores[3] += 1;
                } else {
                    scores[3] += 1;
                    break;
                }
            }

            let scenic_score = scores.iter().product::<usize>();

            if scenic_score > highest_score {
                highest_score = scenic_score;
            }
        }
    }

    highest_score
}

pub fn run() {
    let input = fs::read_to_string("src/inputs/day8").unwrap();

    let grid: Vec<Vec<u32>> = input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let visible_trees = count_visible_trees(&grid);
    let best_scenic_view = best_scenic_view(&grid);

    println!("Visible trees: {}", visible_trees);
    println!("Scenic view: {}", best_scenic_view);

    // println!("part1: {}", test.0); println!("part2: {}", test.1)
}
