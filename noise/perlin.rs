use std::rand::{RngUtil, XorShiftRng};
use std::vec::from_fn;
use super::noisegen::NoiseGen;

pub struct Perlin {
    perm:   ~[uint],
}

impl Perlin {
    pub fn new(seed: Option<(u32, u32, u32, u32)>) -> Perlin {
        let mut rng = match seed {
            Some((x, y, z, w)) => XorShiftRng::new_seeded(x, y, z, w),
            None => XorShiftRng::new(),
        };

        let mut p = Perlin {
            perm: from_fn(512, |i| { i & 255 }),
        };

        for i in range(0, 256) {
            let j = rng.gen_uint_range(0, 256);
            let a = p.perm[i];
            let b = p.perm[j];

            p.perm[i] = b;
            p.perm[i + 256] = b;
            p.perm[j] = a;
            p.perm[j + 256] = a;
        }
        p
    }
}

impl NoiseGen for Perlin {
    fn noise_1d(&self, xin: float) -> float {
        let xi = fast_floor(xin);
        let xo = xin - (xi as float);
        let xf = fade(xo);
        let xw = (xi & 255) as uint;

        let g0 = self.perm[xw];
        let g1 = self.perm[xw + 1];

        let n0 = dot_1d(g0, xo);
        let n1 = dot_1d(g1, xo - 1.0);

        lerp(xf, n0, n1)
    }

    fn noise_2d(&self, xin: float, yin: float) -> float {
        let xi = fast_floor(xin);
        let yi = fast_floor(yin);
        let xo = xin - (xi as float);
        let yo = yin - (yi as float);
        let xf = fade(xo);
        let yf = fade(yo);
        let xw = (xi & 255) as uint;
        let yw = (yi & 255) as uint;

        let g00 = self.perm[xw + self.perm[yw]];
        let g10 = self.perm[xw + 1 + self.perm[yw]];
        let g01 = self.perm[xw + self.perm[yw + 1]];
        let g11 = self.perm[xw + 1 + self.perm[yw + 1]];

        let n00 = dot_2d(g00, xo, yo);
        let n10 = dot_2d(g10, xo - 1.0, yo);
        let n01 = dot_2d(g01, xo, yo - 1.0);
        let n11 = dot_2d(g11, xo - 1.0, yo - 1.0);

        lerp(yf, lerp(xf, n00, n10), lerp(xf, n01, n11))
    }
}

static grad: [[float, ..3], ..16] = [
    [1.0, 1.0, 0.0], [-1.0, 1.0, 0.0], [1.0, -1.0, 0.0], [-1.0, -1.0, 0.0],
    [1.0, 0.0, 1.0], [-1.0, 0.0, 1.0], [1.0, 0.0, -1.0], [-1.0, 0.0, -1.0],
    [0.0, 1.0, 1.0], [0.0, -1.0, 1.0], [0.0, 1.0, -1.0], [0.0, -1.0, -1.0],
    [1.0, 1.0, 0.0], [-1.0, 1.0, 0.0], [0.0, -1.0, 1.0], [0.0, -1.0, -1.0],
];

#[inline(always)]
fn fast_floor(x: float) -> int {
    if x > 0.0 {
        x as int
    } else {
        (x as int) - 1
    }
}

#[inline(always)]
fn fade(t: float) -> float {
    t * t * t * (t * (t * 6.0 - 15.0) + 10.0)
}

#[inline(always)]
fn lerp(t: float, a: float, b: float) -> float {
    a + t * (b - a)
}

#[inline(always)]
fn dot_1d(hash: uint, x: float) -> float {
    let w = hash & 15;
    grad[w][0]*x
}

#[inline(always)]
fn dot_2d(hash: uint, x: float, y: float) -> float {
    let w = hash & 15;
    grad[w][0]*x + grad[w][1]*y
}


