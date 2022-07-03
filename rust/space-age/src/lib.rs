// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug)]
pub struct Duration(f64);

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration((s as f64) / 31557600.0)
    }
}

pub trait Planet {
    const ORBITAL_PERIOD_RATIO: f64;
    fn years_during(d: &Duration) -> f64 {
        d.0 / Self::ORBITAL_PERIOD_RATIO
    }
}

macro_rules! set_orbital_period_ratio {
    ($planet:ident, $ratio:expr) => {
        pub struct $planet;

        impl Planet for $planet {
            const ORBITAL_PERIOD_RATIO: f64 = $ratio;
        }
    };
}

set_orbital_period_ratio!(Mercury, 0.2408467);
set_orbital_period_ratio!(Venus, 0.61519726);
set_orbital_period_ratio!(Earth, 1.0);
set_orbital_period_ratio!(Mars, 1.8808158);
set_orbital_period_ratio!(Jupiter, 11.862615);
set_orbital_period_ratio!(Saturn, 29.447498);
set_orbital_period_ratio!(Uranus, 84.016846);
set_orbital_period_ratio!(Neptune, 164.79132);
