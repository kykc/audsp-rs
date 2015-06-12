
use ::Numeric;
use ::std::vec::Vec;
use ::fcast;
use ::num::Complex;

#[repr(C)]
pub struct DigitalZeroPoleFilter<TReal: Numeric> {
    pub zs: Vec<Complex<TReal>>,
    pub ps: Vec<Complex<TReal>>,
    pub k: TReal
}

#[repr(C)]
pub struct AnalogZeroPoleFilter<TReal: Numeric> {
    pub zs: Vec<Complex<TReal>>,
    pub ps: Vec<Complex<TReal>>,
    pub nw: TReal,
    pub ng: TReal
}

pub fn bilinear_transform<TReal: Numeric>(c:Complex<TReal>, f: TReal) -> Complex<TReal> {

    if c.re == TReal::infinity() {
        return Complex::<TReal>{re: fcast::<TReal>(-1.0), im: TReal::zero()}
    }

    let fc = Complex::<TReal>{re: f, im: TReal::zero()} * c;

    let one = Complex::<TReal>{re: fcast::<TReal>(1.0), im: TReal::zero()};

    (one + fc) / (one - fc)
}

pub fn create_zpk_naive<TReal: Numeric>(order: usize) -> DigitalZeroPoleFilter<TReal> {
    let cz = Complex::<TReal>::new(TReal::zero(), TReal::zero());
    DigitalZeroPoleFilter::<TReal>{zs: vec![cz; order], ps: vec![cz; order], k: TReal::zero()}
}

pub fn square_poly_from_roots<TReal: Numeric>(x1: Complex<TReal>, x2: Complex<TReal>, k: TReal) -> [TReal; 3] {
    [k, (-x1 -x2).re * k, (x1 * x2).re * k]
}

pub fn linear_poly_from_root<TReal: Numeric>(x: Complex<TReal>, k: TReal) -> [TReal; 3] {
    [k, -x.re * k, TReal::zero()]
}

// Assumes that every complex conjugate pair have indeces 2n, 2n + 1 and zero count equals pole count
// TODO: different TSubj, TResult numeric types?
pub fn zp2sos_naive<TReal: Numeric>(zpk: &DigitalZeroPoleFilter<TReal>) -> Vec<::sos::SecondOrderSection<TReal>> {
    let num_sos = zpk.ps.len() / 2;
    let num_fos = zpk.ps.len() % 2;

    let mut result = vec![::sos::nil::<TReal>(); num_sos + num_fos];

    for i in 0..num_sos {
        result[i].acs = square_poly_from_roots(zpk.ps[2 * i], zpk.ps[2 * i + 1], fcast::<TReal>(1.0));
        result[i].bcs = square_poly_from_roots(zpk.zs[2 * i], zpk.ps[2 * i + 1], zpk.k);
    }

    for i in 0..num_fos {
        result[num_sos + i].acs = linear_poly_from_root(zpk.ps[num_sos * 2 + 1 + i], TReal::one());
        result[num_sos + i].bcs = linear_poly_from_root(zpk.zs[num_sos * 2 + 1 + i], zpk.k);
    }

    result
}
