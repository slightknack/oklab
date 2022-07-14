// translation of the public domain code avaliable here:
// https://bottosson.github.io/posts/oklab/

pub struct Oklab {
    pub l: f64,
    pub a: f64,
    pub b: f64,
}

impl Oklab {
    pub fn new(l: f64, a: f64, b: f64) -> Oklab {
        Oklab { l, a, b }
    }

    pub fn to_linear_srgb(self) -> LinearSrgb {
        let c = self;
        let l_ = c.l + 0.3963377774 * c.a + 0.2158037573 * c.b;
        let m_ = c.l - 0.1055613458 * c.a - 0.0638541728 * c.b;
        let s_ = c.l - 0.0894841775 * c.a - 1.2914855480 * c.b;

        let l = l_ * l_ * l_;
        let m = m_ * m_ * m_;
        let s = s_ * s_ * s_;

        LinearSrgb::new(
            0.0 + 4.0767416621 * l - 3.3077115913 * m + 0.2309699292 * s,
            0.0 - 1.2684380046 * l + 2.6097574011 * m - 0.3413193965 * s,
            0.0 - 0.0041960863 * l - 0.7034186147 * m + 1.7076147010 * s,
        )
    }
}

pub struct LinearSrgb {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl LinearSrgb {
    pub fn new(r: f64, g: f64, b: f64) -> LinearSrgb {
        LinearSrgb { r, g, b }
    }

    pub fn to_oklab(self) -> Oklab {
        let c = self;
        let l = 0.4122214708 * c.r + 0.5363325363 * c.g + 0.0514459929 * c.b;
        let m = 0.2119034982 * c.r + 0.6806995451 * c.g + 0.1073969566 * c.b;
        let s = 0.0883024619 * c.r + 0.2817188376 * c.g + 0.6299787005 * c.b;

        let l_ = l.powf(1.0 / 3.0);
        let m_ = m.powf(1.0 / 3.0);
        let s_ = s.powf(1.0 / 3.0);

        Oklab::new(
            0.2104542553 * l_ + 0.7936177850 * m_ - 0.0040720468 * s_,
            1.9779984951 * l_ - 2.4285922050 * m_ + 0.4505937099 * s_,
            0.0259040371 * l_ + 0.7827717662 * m_ - 0.8086757660 * s_,
        )
    }
}
