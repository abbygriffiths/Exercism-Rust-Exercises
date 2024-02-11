const SECONDS_PER_YEAR_ON_EARTH: f64 = 31557600.0;

#[derive(Debug)]
pub struct Duration(u64);

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Self(s)
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64 {
        todo!("convert a duration ({d:?}) to the number of years on this planet for that duration");
    }
}

macro_rules! define_planet {
    ($planet:ident, $orbital_period:expr) => {
        pub struct $planet;

        impl Planet for $planet {
            fn years_during(d: &Duration) -> f64 {
                (d.0 as f64) / ($orbital_period * SECONDS_PER_YEAR_ON_EARTH)
            }
        }
    };
}

define_planet!(Mercury, 0.2408467);
define_planet!(Venus, 0.61519726);
define_planet!(Earth, 1.0);
define_planet!(Mars, 1.8808156);
define_planet!(Jupiter, 11.862515);
define_planet!(Saturn, 29.447498);
define_planet!(Uranus, 84.016846);
define_planet!(Neptune, 164.79132);
