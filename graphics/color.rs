#[deriving(Clone)]
#[pack]
pub struct Color {
    r:  u8,
    g:  u8,
    b:  u8,
    a:  u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color {
            r:  r,
            g:  g,
            b:  b,
            a:  a,
        }
    }

    pub fn blend_channel(c0: u8, c1: u8, alpha: float) -> u8 {
        let c0r = (c0 as float) / 255.0;
        let c1r = (c1 as float) / 255.0;
        (((c1r * alpha) + (c0r * (1.0 - alpha))) * 255.0) as u8
    }

    pub fn blend(c0: Color, c1: Color, alpha: float) -> Color {
        Color {
            r:  Color::blend_channel(c0.r, c1.r, alpha),
            g:  Color::blend_channel(c0.g, c1.g, alpha),
            b:  Color::blend_channel(c0.b, c1.b, alpha),
            a:  Color::blend_channel(c0.a, c1.a, alpha),
        }
    }
}
