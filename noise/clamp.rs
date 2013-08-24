use super::noisegen::NoiseGen;

pub struct Clamp<'self, T> {
    source: &'self T,
    min:    float,
    max:    float,
}

impl<'self, T: NoiseGen> Clamp<'self, T> {
    pub fn new<'r>(source: &'r T, min: float, max: float) -> Clamp<'r, T> {
        Clamp {
            source: source,
            min:    min,
            max:    max,
        }
    }
}

impl<'self, T: NoiseGen> NoiseGen for Clamp<'self, T> {
    fn noise_1d(&self, xin: float) -> float {
        let res = self.source.noise_1d(xin);

        if res < self.min {
            self.min
        } else if res > self.max {
            self.max
        } else {
            res
        }
    }

    fn noise_2d(&self, xin: float, yin: float) -> float {
        let res = self.source.noise_2d(xin, yin);

        if res < self.min {
            self.min
        } else if res > self.max {
            self.max
        } else {
            res
        }
    }
}
