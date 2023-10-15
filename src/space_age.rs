// Instructions
//
// Given an age in seconds, calculate how old someone would be on:

// Mercury: orbital period 0.2408467 Earth years
// Venus: orbital period 0.61519726 Earth years
// Earth: orbital period 1.0 Earth years, 365.25 Earth days, or 31557600 seconds
// Mars: orbital period 1.8808158 Earth years
// Jupiter: orbital period 11.862615 Earth years
// Saturn: orbital period 29.447498 Earth years
// Uranus: orbital period 84.016846 Earth years
// Neptune: orbital period 164.79132 Earth years
//
// So if you were told someone were 1,000,000,000 seconds old, you should be able to say that they're 31.69 Earth-years old.

const EARTH_YEAR_IN_SECONDS: u64 = 31557600;

#[derive(Debug)]
pub struct Duration(f64);

pub struct OrbitalPeriod(f64);

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration(s as f64 / EARTH_YEAR_IN_SECONDS as f64)
    }
}

pub trait Planet {
    const ORBITAL_PERIOD: OrbitalPeriod;

    fn years_during(Duration(duration): &Duration) -> f64 {
        let OrbitalPeriod(orbital_period) = Self::ORBITAL_PERIOD;

        duration / orbital_period
    }
}

macro_rules! planet {
    ($name:ident, $orbital_period:expr) => {
        pub struct $name;
        impl Planet for $name {
            const ORBITAL_PERIOD: OrbitalPeriod = $orbital_period;
        }
    };
}

planet!(Mercury, OrbitalPeriod(0.2408467));
planet!(Venus, OrbitalPeriod(0.61519726));
planet!(Earth, OrbitalPeriod(1.0));
planet!(Mars, OrbitalPeriod(1.8808158));
planet!(Jupiter, OrbitalPeriod(11.862615));
planet!(Saturn, OrbitalPeriod(29.447498));
planet!(Uranus, OrbitalPeriod(84.016846));
planet!(Neptune, OrbitalPeriod(164.79132));

#[cfg(test)]
mod test {
    use super::*;

    fn assert_in_delta(expected: f64, actual: f64) {
        let diff: f64 = (expected - actual).abs();
        let delta: f64 = 0.01;
        if diff > delta {
            panic!(
                "Your result of {} should be within {} of the expected result {}",
                actual, delta, expected
            )
        }
    }

    #[test]
    fn earth_age() {
        let duration = Duration::from(1_000_000_000);
        assert_in_delta(31.69, Earth::years_during(&duration));
    }

    #[test]
    fn mercury_age() {
        let duration = Duration::from(2_134_835_688);
        assert_in_delta(280.88, Mercury::years_during(&duration));
    }

    #[test]
    fn venus_age() {
        let duration = Duration::from(189_839_836);
        assert_in_delta(9.78, Venus::years_during(&duration));
    }

    #[test]
    fn mars_age() {
        let duration = Duration::from(2_129_871_239);
        assert_in_delta(35.88, Mars::years_during(&duration));
    }

    #[test]
    fn jupiter_age() {
        let duration = Duration::from(901_876_382);
        assert_in_delta(2.41, Jupiter::years_during(&duration));
    }

    #[test]
    fn saturn_age() {
        let duration = Duration::from(2_000_000_000);
        assert_in_delta(2.15, Saturn::years_during(&duration));
    }

    #[test]
    fn uranus_age() {
        let duration = Duration::from(1_210_123_456);
        assert_in_delta(0.46, Uranus::years_during(&duration));
    }

    #[test]
    fn neptune_age() {
        let duration = Duration::from(1_821_023_456);
        assert_in_delta(0.35, Neptune::years_during(&duration));
    }
}
