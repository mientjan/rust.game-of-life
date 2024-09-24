use sdl2::rect::Rect;
use sdl2::render::{Texture};
use crate::objects::Dot;
use crate::properties::COLORS;

pub fn update_texture(
    texture: &mut Texture,
    changes: &Vec<Dot>,
    changes_length: &u32,
    screen: &Rect
) {
    let width = screen.width() as usize;

    texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {

        for i in 0..*changes_length as usize {
            let index = changes[i].index as usize;
            let x = index % width;
            let mut y = index / width | 0;
            y = y * pitch;
            let offset = y + x * 3;  // Each pixel is 3 bytes (RGB24 format)

            let cell_color = if changes[i].state { COLORS.white } else { COLORS.black };

            buffer[offset] = cell_color.r;
            buffer[offset + 1] = cell_color.g;   // G
            buffer[offset + 2] = cell_color.b;  // B
        }
    }).unwrap();
}