macro_rules! impl_years_during {
    ($i:ident, $p:expr) => {
        pub struct $i;
        impl Planet for $i {
            fn years_during(d: &Duration) -> f64 {
                d.0 / 31557600 as f64 / ($p)
            }
        }
    };
}

#[derive(Debug)]
pub struct Duration(f64);

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration(s as f64)
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64;
}

impl_years_during!(Mercury, 0.2408467);
impl_years_during!(Venus, 0.61519726);
impl_years_during!(Earth, 1.0);
impl_years_during!(Mars, 1.8808158);
impl_years_during!(Jupiter, 11.862615);
impl_years_during!(Saturn, 29.447498);
impl_years_during!(Uranus, 84.016846);
impl_years_during!(Neptune, 164.79132);