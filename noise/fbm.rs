use super::noisegen::NoiseGen;

pub struct Fbm<'self, T> {
    source:         &'self T,
    octaves:        uint,
    frequency:      float,
    lacunarity:     float,
    persistence:    float,
}

impl<'self, T: NoiseGen> Fbm<'self, T> {
    pub fn new<'r>(source: &'r T, octaves: uint, frequency: float,
        lacunarity: float, persistence: float) -> Fbm<'r, T> {
        Fbm {
            source:         source,
            octaves:        octaves,
            frequency:      frequency,
            lacunarity:     lacunarity,
            persistence:    persistence,
        }
    }
}

impl<'self, T: NoiseGen> NoiseGen for Fbm<'self, T> {
    fn noise_1d(&self, xin: float) -> float {
        let mut freq = self.frequency;
        let mut amp = 1.0;
        let mut res = 0.0;

        do self.octaves.times {
            res += self.source.noise_1d(xin * freq) * amp;
            freq *= self.lacunarity;
            amp *= self.persistence;
        }

        res
    }

    fn noise_2d(&self, xin: float, yin: float) -> float {
        let mut freq = self.frequency;
        let mut amp = 1.0;
        let mut res = 0.0;

        do self.octaves.times {
            res += self.source.noise_2d(xin * freq, yin * freq) * amp;
            freq *= self.lacunarity;
            amp *= self.persistence;
        }

        res
    }
}

