use sdl2::pixels::PixelFormatEnum;
use sdl2::render::TextureCreator;
use sdl2::video::WindowContext;

pub fn isPerfectSquare(x: f64) -> bool
{
    let s = (x as f64).sqrt();
    return (s * s == x);
}
 
// Returns true if n is a Fibonacci Number, else false
pub fn isFibonacci(n: usize) -> bool
{
    // n is Fibonacci if one of 5*n*n + 4 or 5*n*n - 4 or
    // both is a perfect square

    let mut v0: usize = (5 * n * n);
    let mut v1 = (5 * n * n);

    if v1 < 4 {
        return false;
    }

    v0 += 4;
    v1 -= 4;

    return isPerfectSquare(v0 as f64) || isPerfectSquare(v1 as f64);
}

pub fn create_grid_texture(texture_creator: &TextureCreator<WindowContext>, width: u32, height: u32) -> sdl2::render::Texture {
    return texture_creator
        .create_texture_streaming(PixelFormatEnum::RGB24, width, height)
        .unwrap();
}