extern crate num;

pub mod sos;
pub mod cinterop;
pub mod zpk;

pub trait Numeric: num::Float + Default {
    fn pi(Self) -> Self;
}

impl Numeric for f32 {
    fn pi(x: f32) -> f32 {
        x * 3.14159265359
        //f32::consts::PI

    }
}

#[allow(dead_code)]
pub fn rn<TReal: Numeric, TSubj: Numeric>(x: TSubj) -> TReal {
    TReal::from(x).unwrap()
}

pub fn cast<T: num::NumCast, U: num::NumCast>(n: T) -> Option<U> {
    num::NumCast::from(n)
}

pub fn fcast<U: num::NumCast>(n: f32) -> U {
    num::NumCast::from(n).unwrap()
}

impl Numeric for f64 {
    fn pi(x: f64) -> f64 {
        x * 3.14159265359
    }
}

pub trait Filter<TReal: Numeric> {
    fn process(&mut self, input: &[TReal], output: &mut[TReal]);
    fn clear_state(&mut self);
}

pub trait BiQuadFilter<TReal: Numeric> : Filter<TReal> {
    fn init(&mut self, acs: &[TReal; 3], bcs: &[TReal; 3]);
}
