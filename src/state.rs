use crate::utils;

// only works if ther grid is rectangular
pub fn update_grid(grid: &mut Vec<Vec<bool>>) {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut new_grid = vec![vec![false; cols]; rows];

    for i in 0..rows {
        for j in 0..cols {
            let live_neighbors = count_live_neighbors(&grid, i, j);

            if grid[i][j] {
                // Any live cell with fewer than two live neighbors dies
                // due to underpopulation.
                if live_neighbors < 2 {
                    new_grid[i][j] = false;
                }
                // Any live cell with two or three live neighbors lives
                // on to the next generation.
                else if live_neighbors == 2 || live_neighbors == 3 {
                    new_grid[i][j] = true;
                }
                // Any live cell with more than three live neighbors dies
                // due to overpopulation.
                else {
                    new_grid[i][j] = false;
                }
            } else {
                // Any dead cell with exactly three live neighbors becomes
                // a live cell, as if by reproduction.
                if live_neighbors == 3 {
                    new_grid[i][j] = true;
                }
            }
        }
    }

    *grid = new_grid;
}

fn count_live_neighbors(grid: &Vec<Vec<bool>>, row: usize, col: usize) -> usize {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut count = 0;

    for i in (row as isize - 1)..=(row as isize + 1) {
        for j in (col as isize - 1)..=(col as isize + 1) {
            if i >= 0 && i < rows as isize && j >= 0 && j < cols as isize && !(i == row as isize && j == col as isize) {
                if grid[i as usize][j as usize] {
                    count += 1;
                }
            }
        }
    }

    return count;
}

pub fn prepare_grid(grid: &mut Vec<Vec<bool>>) {
    let rows = grid.len();
    let cols = grid[0].len();

    for i in 0..rows {
        for j in 0..cols {
            grid[i][j] = utils::isFibonacci((i * cols + j).try_into().unwrap());
        }
    }
}