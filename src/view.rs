use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub fn draw(grid: &[Vec<bool>], canvas: &mut Canvas<Window>) {
    // Clear the canvas
    canvas.set_draw_color(Color::BLACK);
    canvas.clear();

    // Calculate the size of each cell based on the canvas dimensions and grid size
    let cell_width = canvas.viewport().width() as usize / grid[0].len();
    let cell_height = canvas.viewport().height() as usize / grid.len();

    // Iterate over each cell in the grid
    for (row, row_cells) in grid.iter().enumerate() {
        for (col, &cell) in row_cells.iter().enumerate() {
            // Calculate the position and size of the cell rectangle
            let x = (col * cell_width) as i32;
            let y = (row * cell_height) as i32;
            let width = cell_width as u32;
            let height = cell_height as u32;

            // Set the color of the cell based on its state (alive or dead)
            let color = if cell { Color::WHITE } else { Color::BLACK };

            // Draw the cell rectangle on the canvas
            canvas.set_draw_color(color);
            canvas.fill_rect(Rect::new(x, y, width, height)).unwrap();
        }
    }

}