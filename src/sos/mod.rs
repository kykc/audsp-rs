use ::Numeric;
use ::fcast;

pub mod df_one;

// TODO: implement clone trait
#[repr(C)]
pub struct SecondOrderSection<TReal> {
    pub acs: [TReal; 3],
    pub bcs: [TReal; 3]
}

impl<TReal: Numeric> Clone for SecondOrderSection<TReal> {
    fn clone(&self) -> Self {
        SecondOrderSection::<TReal>{acs: self.acs, bcs: self.bcs}
    }
}

#[inline]
pub fn nil<TReal: Numeric>() -> SecondOrderSection<TReal> {
    SecondOrderSection{acs: [TReal::zero(), TReal::zero(), TReal::zero()], bcs: [TReal::zero(), TReal::zero(), TReal::zero()]}
}

#[inline]
pub fn one<TReal: Numeric>() -> SecondOrderSection<TReal> {
    SecondOrderSection{acs: [TReal::zero(), TReal::zero(), TReal::zero()], bcs: [TReal::one(), TReal::zero(), TReal::zero()]}
}

#[inline]
pub fn clone<TReal: Numeric>(acs: &[TReal; 3], bcs: &[TReal; 3]) -> SecondOrderSection<TReal> {
    SecondOrderSection{acs: *acs, bcs: *bcs}
}

#[inline]
pub fn scale<TReal: Numeric>(subj: &mut SecondOrderSection<TReal>) {
    let scale = subj.acs[0];

    for i in 0..3 {
        subj.acs[i] = subj.acs[i] / scale;
        subj.bcs[i] = subj.bcs[i] / scale;
    }
}

pub fn peaking<TReal: Numeric>(subj: &mut SecondOrderSection<TReal>, q: TReal, gain: TReal, f0: TReal, fs: TReal) {
    /*double A = sqrt(std::pow(10, (dBgain / 20)));
    double w0 = 2 * M_PI * f0 / Fs;
    double alpha = sin(w0) / (2 * Q);

    std::vector<double> as;
    std::vector<double> bs;

    as.resize(3, 0.0);
    bs.resize(3, 0.0);

    bs[0] = 1 + alpha*A;
    bs[1] = -2 * cos(w0);
    bs[2] = 1 - alpha*A;
    as[0] = 1 + alpha / A;
    as[1] = -2 * cos(w0);
    as[2] = 1 - alpha / A;*/

    let c10: TReal = TReal::from(10.0).unwrap();
    let c20: TReal = TReal::from(20.0).unwrap();
    let c2: TReal = TReal::from(2.0).unwrap();

    let a: TReal = TReal::sqrt(TReal::powf(c10, gain / c20));
    let w0: TReal = TReal::pi(c2) * f0 / fs;
    let alpha: TReal = TReal::sin(w0) / (c2 * q);

    subj.bcs[0] = fcast::<TReal>(1.0) + alpha * a;
    subj.bcs[1] = fcast::<TReal>(-2.0) * TReal::cos(w0);
    subj.bcs[2] = fcast::<TReal>(1.0) - alpha * a;

    subj.acs[0] = fcast::<TReal>(1.0) + alpha / a;
    subj.acs[1] = fcast::<TReal>(-2.0) * TReal::cos(w0);
    subj.acs[2] = fcast::<TReal>(1.0) - alpha / a;

    scale(subj);

    /*for i in 0..3 {
        println!("{}", cast::<TReal, f32>(self.acs[i]).unwrap());
    }

    for i in 0..3 {
        println!("{}", cast::<TReal, f32>(self.bcs[i]).unwrap());
    }*/
}
