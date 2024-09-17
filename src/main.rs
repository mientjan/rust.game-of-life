
#![allow(warnings)]

// use log::info;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
// use sdl2::video;
// use sdl2::render::WindowCanvas;
// use sdl2::video::Window;
use std::time::Duration;

mod state;
mod utils;
// mod view;

fn main() {
    
    const PADDING_PERCENTAGE: f32 = 0.2;

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let screen = video_subsystem.display_bounds(0).unwrap();

    // let width: u32 =  screen.width() as u32;
    // let height: u32 =  screen.height() as u32;

    let window_width = screen.width() as f32;
    let window_height = screen.height() as f32;
    let width = (window_width - (window_width * (2.0 * PADDING_PERCENTAGE))) as u32;
    let height = (window_height - (window_height * (2.0 * PADDING_PERCENTAGE))) as u32;
    
    // return ();

    let window = video_subsystem
        .window("Game of Life", width, height)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.set_draw_color(Color::RGB(255, 255, 255));

    // Draw the game area with padding
    let game_area = Rect::new(
        0,
        0,
        width,
        height,
    );
    canvas.draw_rect(game_area).unwrap();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();

    // game grid
    let mut grid = vec![vec![false; width as usize]; height as usize];
    let mut loop_count:u32 = 0;
    // let ttf_context = sdl2::ttf::init().unwrap();
    // let font = ttf_context.load_font("assets/Roboto-Regular.ttf", 24).unwrap();

    let mut draw_rect = Rect::new(0, 0, 1, 1);

    state::prepare_grid(&mut grid);
    let padding = 0;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        loop_count += 1;

        // Game logic goes here
        state::update_grid(&mut grid);

        let color_black = Color::RGB(0, 0, 0);
        let color_white = Color::RGB(255, 255, 255);

        // Clear the canvas
        canvas.set_draw_color(color_black);
        canvas.clear();

        // Draw the game area with padding
        canvas.set_draw_color(color_white);
        canvas.draw_rect(game_area).unwrap();

        draw_rect.set_x(loop_count as i32 % width as i32);
        draw_rect.set_y(loop_count as i32 % height as i32);
        canvas.fill_rect(draw_rect).unwrap();

        // Draw the grid
        for (row_index, row) in grid.iter().enumerate() {
            for (col_index, cell) in row.iter().enumerate() {
                draw_rect.set_x((padding as i32) + (col_index as i32));
                    draw_rect.set_y((padding as i32) + (row_index as i32));

                    // println!("draw_rect: {:?}, cell: {}", draw_rect, cell);

                if *cell {
                    canvas.set_draw_color(color_white);
                } else {
                    canvas.set_draw_color(color_black);
                    
                }
                canvas.fill_rect(draw_rect).unwrap();
            }
        }
        // view::draw(&grid, &mut canvas);

        // Draw the loop count
        // view::draw_text(&mut canvas, &font, &format!("Loop: {}", loop_count), 10, 10);

        // Draw game objects here
        canvas.present();
        canvas.clear();

        // Delay to control the frame rate
        std::thread::sleep(Duration::from_millis(16));
    }

}
