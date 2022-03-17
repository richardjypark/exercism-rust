// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.
const MERCURY_EARTH_YEARS: f64 = 0.2408467;
const VENUS_EARTH_YEARS: f64 = 0.61519726;
const MARS_EARTH_YEARS: f64 = 1.8808158;
const JUPITER_EARTH_YEARS: f64 = 11.862615;
const SATURN_EARTH_YEARS: f64 = 29.447498;
const URANUS_EARTH_YEARS: f64 = 84.016846;
const NEPTUNE_EARTH_YEARS: f64 = 164.79132;

#[derive(Debug)]
pub struct Duration {
    // earth years
    time: f64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Self {
            time: (s as f64) / 31557600.0,
        }
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64 {
        d.time
    }
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

impl Planet for Mercury {
    fn years_during(d: &Duration) -> f64 {
        d.time / MERCURY_EARTH_YEARS
    }
}
impl Planet for Venus {
    fn years_during(d: &Duration) -> f64 {
        d.time / VENUS_EARTH_YEARS
    }
}
impl Planet for Earth {}
impl Planet for Mars {
    fn years_during(d: &Duration) -> f64 {
        d.time / MARS_EARTH_YEARS
    }
}
impl Planet for Jupiter {
    fn years_during(d: &Duration) -> f64 {
        d.time / JUPITER_EARTH_YEARS
    }
}
impl Planet for Saturn {
    fn years_during(d: &Duration) -> f64 {
        d.time / SATURN_EARTH_YEARS
    }
}
impl Planet for Uranus {
    fn years_during(d: &Duration) -> f64 {
        d.time / URANUS_EARTH_YEARS
    }
}
impl Planet for Neptune {
    fn years_during(d: &Duration) -> f64 {
        d.time / NEPTUNE_EARTH_YEARS
    }
}
