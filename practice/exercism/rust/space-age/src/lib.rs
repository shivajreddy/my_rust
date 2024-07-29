#[derive(Debug)]
pub struct Duration {
    in_earth_years: f64,
}

const EARTH_YEAR_IN_SECONDS: u32 = 31_557_600;

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration {
            in_earth_years: s as f64 / EARTH_YEAR_IN_SECONDS as f64,
        }
    }
}

pub trait Planet {
    const YEARS_IN_EARTH_YEARS: f64;
    fn years_during(d: &Duration) -> f64 {
        d.in_earth_years / Self::YEARS_IN_EARTH_YEARS
    }
}

macro_rules! impl_planet {
    ($planet:ident, $years_in_earth_years:expr) => {
        pub struct $planet;

        impl Planet for $planet {
            const YEARS_IN_EARTH_YEARS: f64 = $years_in_earth_years;
        }
    };
}

impl_planet!(Mercury, 0.2408467);
impl_planet!(Venus, 0.61519726);
impl_planet!(Earth, 1.0);
impl_planet!(Mars, 1.8808158);
impl_planet!(Jupiter, 11.862615);
impl_planet!(Saturn, 29.447498);
impl_planet!(Uranus, 84.016846);
impl_planet!(Neptune, 164.79132);
