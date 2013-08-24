use std::vec::from_elem;
use super::color::Color;

pub struct Image {
    width:  uint,
    height: uint,
    pixels: ~[Color],
}

impl Image {
    pub fn new(width: uint, height: uint) -> Image {
        Image {
            width:  width,
            height: height,
            pixels: from_elem(width * height, Color::new(0,0,0,0)),
        }
    }

    #[inline(always)]
    pub fn set_pixel(&mut self, x: uint, y: uint, c: Color) {
        self.pixels[y * self.width + x] = c;
    }

    #[inline(always)]
    pub fn get_pixel(&self, x: uint, y: uint) -> Color {
        self.pixels[y * self.width + x]
    }
}

