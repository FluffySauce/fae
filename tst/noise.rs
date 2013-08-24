extern mod fae;
use fae::noise::NoiseGen;
use fae::noise::Perlin;
use fae::noise::Fbm;
use fae::noise::Clamp;

fn main() {
    let p = Perlin::new(None);
    let f = Fbm::new(&p, 6, 1.0, 2.0, 0.8);
    let c = Clamp::new(&f, -1.0, 1.0);

    for y in range(0, 11) {
        for x in range(0, 11) {
            let xin = (x as float) / 10.0;
            let yin = (y as float) / 10.0;
            let res = c.noise_2d(xin, yin);

            printf!("% 0.2f ", res);
        }
        println("");
    }
}
