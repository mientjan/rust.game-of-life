
#![allow(warnings)]

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use std::time::Duration;

mod state;
mod objects;
mod utils;
mod view;
mod properties;

use crate::objects::{Dot};
use crate::properties::COLORS;
use crate::utils::{create_grid_texture};

fn main() {
    
    const PADDING_PERCENTAGE: f32 = 0.1;

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let screen = video_subsystem.display_bounds(0).unwrap();

    let game_area = Rect::new(
        0,
        0,
        ((screen.width() as f32) - ((screen.width() as f32) * (2.0 * PADDING_PERCENTAGE))) as u32,
        ((screen.height() as f32) - ((screen.height() as f32) * (2.0 * PADDING_PERCENTAGE))) as u32,
    );

    let window = video_subsystem
        .window("Game of Life", game_area.width(), game_area.height())
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(COLORS.black);
    canvas.clear();

    canvas.set_draw_color(COLORS.white);

    canvas.draw_rect(game_area).unwrap();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let size = game_area.width() * game_area.height();

    // game grid
    let mut grid = vec![false; size as usize];
    let dot = Dot::new(0, false);
    let mut grid_changes = vec![dot; size as usize];
    let mut grid_changes_length: u32 = 0;

    let texture_creator = canvas.texture_creator();
    let mut texture = create_grid_texture(&texture_creator, game_area.width(), game_area.height());

    state::init(&mut grid_changes, &mut grid_changes_length, game_area.width(), game_area.height());
    view::update_texture(&mut texture, &grid_changes, &grid_changes_length, &game_area);

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

        state::apply_changes(&mut grid, &grid_changes, &mut grid_changes_length);
        state::update(&mut grid, &mut grid_changes, &mut grid_changes_length, game_area.width(), game_area.height(), size);
        view::update_texture(&mut texture, &grid_changes, &grid_changes_length, &game_area);

        // Clear the canvas
        canvas.clear();

        // Copy the updated texture onto the canvas
        canvas.copy(&texture, game_area, game_area).expect("TODO: panic message");

        // Present the updated canvas to the screen
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 1000));
    }


}
