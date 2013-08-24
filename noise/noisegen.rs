pub trait NoiseGen {
    fn noise_1d(&self, xin: float) -> float;
    fn noise_2d(&self, xin: float, yin: float) -> float;
}

