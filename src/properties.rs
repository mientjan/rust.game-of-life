use crate::objects::Colors;

pub(crate) static COLORS: Colors = Colors {
    white: sdl2::pixels::Color::RGB(255, 255, 255),
    black: sdl2::pixels::Color::RGB(0, 0, 0),
};