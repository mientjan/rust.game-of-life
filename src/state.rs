use crate::{objects::Dot, utils};
use crate::utils::isFibonacci;

// only works if ther grid is rectangular
pub fn update(grid: &mut Vec<bool>, changes: &mut Vec<Dot>, changes_length: &mut u32, width: u32, height: u32, size: u32 ) {

    let mut length: usize = 0;
    for i in 0..size {
        let x = i % width;
        let y = i / width | 0;
        
        let live_neighbors = count_live_neighbors(&grid, width, height, x, y);
        let val = grid[i as usize];

        if val {
            if(!(live_neighbors == 2 || live_neighbors == 3)){
                changes[length].index = i;
                changes[length].state = false;
                length += 1;
            }
        } else if (live_neighbors == 3) {
            if(val == false){
                changes[length].index = i;
                changes[length].state = true;
                length += 1;
            }
        }
    }

    *changes_length = length as u32;
}

fn count_live_neighbors(grid: &Vec<bool>, width: u32, height: u32, x: u32, y: u32) -> u8 {
    let mut live_neighbors = 0;

    // Check the top-left neighbor
    if x > 0 && y > 0 && grid[(y - 1) as usize * width as usize + (x - 1) as usize] {
        live_neighbors += 1;
    }

    // Check the top neighbor
    if y > 0 && grid[(y - 1) as usize * width as usize + x as usize] {
        live_neighbors += 1;
    }

    // Check the top-right neighbor
    if x < width - 1 && y > 0 && grid[(y - 1) as usize * width as usize + (x + 1) as usize] {
        live_neighbors += 1;
    }

    // Check the left neighbor
    if x > 0 && grid[y as usize * width as usize + (x - 1) as usize] {
        live_neighbors += 1;
    }

    // Check the right neighbor
    if x < width - 1 && grid[y as usize * width as usize + (x + 1) as usize] {
        live_neighbors += 1;
    }

    // Check the bottom-left neighbor
    if x > 0 && y < height - 1 && grid[(y + 1) as usize * width as usize + (x - 1) as usize] {
        live_neighbors += 1;
    }

    // Check the bottom neighbor
    if y < height - 1 && grid[(y + 1) as usize * width as usize + x as usize] {
        live_neighbors += 1;
    }

    // Check the bottom-right neighbor
    if x < width - 1 && y < height - 1 && grid[(y + 1) as usize * width as usize + (x + 1) as usize] {
        live_neighbors += 1;
    }

    return live_neighbors;
}

pub fn init(changes: &mut Vec<Dot>, changes_length: &mut u32, width: u32, height: u32 ) {
    let size = width * height;
    for i in 0..size {
        let x = i % width;
        // let y = i / size | 0;

        changes[i as usize].index = i;
        // changes[i as usize].state = if x > 10 && x < 20 { true } else { false };
        changes[i as usize].state = isFibonacci(i as usize);
    }

    *changes_length = size as u32;
}

pub fn apply_changes(grid: &mut Vec<bool>, changes: &Vec<Dot>, changes_length: &mut u32) {
    for i in 0..(*changes_length) {
        grid[changes[i as usize].index as usize] = changes[i as usize].state;
    }

    *changes_length = 0;
}