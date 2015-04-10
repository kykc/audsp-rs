extern crate num;
extern crate libc;

use num::Num;
use num::Zero;
use std::cmp;


use libc::{c_void, size_t, malloc, free};
use std::mem;
use std::ptr;

pub struct SecondOrderSection<TReal> {
    pub acs: [TReal; 3],
    pub bcs: [TReal; 3]
}

pub struct DFOneState<TReal> {
    pub xvs: [TReal; 3],
    pub yvs: [TReal; 3]
}

pub trait BiQuadFilter<TReal: Num + Copy> {
    fn process(&mut self, input: & [TReal], output: &mut[TReal]);
    fn init(&mut self, acs: &[TReal; 3], bcs: &[TReal; 3]);
}

pub struct DFOneBiQuad<TReal: Num + Copy> {
    pub coeffs: SecondOrderSection<TReal>,
    pub state: DFOneState<TReal>
}

impl<TReal: Num + Copy> SecondOrderSection<TReal> {
    #[inline]
    pub fn nil() -> SecondOrderSection<TReal> {
        SecondOrderSection{acs: [TReal::zero(), TReal::zero(), TReal::zero()], bcs: [TReal::zero(), TReal::zero(), TReal::zero()]}
    }

    pub fn one() -> SecondOrderSection<TReal> {
        SecondOrderSection{acs: [TReal::zero(), TReal::zero(), TReal::zero()], bcs: [TReal::one(), TReal::zero(), TReal::zero()]}
    }

    pub fn snatch(acs: &[TReal; 3], bcs: &[TReal; 3]) -> SecondOrderSection<TReal> {
        SecondOrderSection{acs: *acs, bcs: *bcs}
    }
}

impl<TReal: Num> DFOneState<TReal> {
    pub fn nil() -> DFOneState<TReal> {
        DFOneState{xvs: [TReal::zero(), TReal::zero(), TReal::zero()], yvs: [TReal::zero(), TReal::zero(), TReal::zero()]}
    }
}

impl<TReal: Num + Copy> DFOneBiQuad<TReal> {
    pub fn one() -> DFOneBiQuad<TReal> {
        DFOneBiQuad{coeffs: SecondOrderSection::<TReal>::one(), state: DFOneState::<TReal>::nil()}
    }

    pub fn create(acs: &[TReal; 3], bcs: &[TReal; 3]) -> DFOneBiQuad<TReal> {
        DFOneBiQuad{coeffs: SecondOrderSection::<TReal>::snatch(acs, bcs), state: DFOneState::<TReal>::nil()}
    }
}

impl<TReal: Num + Copy> BiQuadFilter<TReal> for DFOneBiQuad<TReal> {

    fn init(&mut self, acs: &[TReal; 3], bcs: &[TReal; 3]) {
        self.coeffs.acs = *acs;
        self.coeffs.bcs = *bcs;
    }

    #[inline]
    fn process(&mut self, input: & [TReal], output: &mut [TReal]) {
        for i in 0..cmp::min(input.len(), output.len()) {
            self.state.xvs[2] = self.state.xvs[1];
            self.state.xvs[1] = self.state.xvs[0];
            self.state.xvs[0] = input[i];
            self.state.yvs[2] = self.state.yvs[1];
            self.state.yvs[1] = self.state.yvs[0];
            self.state.yvs[0] = self.state.xvs[0] * self.coeffs.bcs[0] + self.state.xvs[1] * self.coeffs.bcs[1] + self.state.xvs[2] * self.coeffs.bcs[2] -
                self.state.yvs[1] * self.coeffs.acs[1] - self.state.yvs[2] * self.coeffs.acs[2];
            output[i] = self.state.yvs[0];
        }
    }
}

fn process_iir_cascade<TReal: Num + Copy>(filters: &mut [&mut BiQuadFilter<TReal>], input: & [TReal], output: &mut [TReal]) {
    for filter in filters {
        filter.process(input, output);
    }
}

#[no_mangle]
pub unsafe extern fn c_process32(input: *mut f32, output: *mut f32, acs: *const f32, bcs: *const f32, buffer_length: *const usize) {

    let mut filter = DFOneBiQuad::<f32>::one();

    for i in 0..3i32 {
        filter.coeffs.acs[i] = *acs.offset(i);
    }

    for i in 0..*buffer_length {

    }
}

fn main() {

        let mut buffer: Vec<f32> = vec![0.0; 512];
        buffer[0] = 1.0;
        let mut output: Vec<f32> = vec![0.0; 512];

        let mut filter = DFOneBiQuad::<f32>::one();
        let acs: [f32; 3] = [1.0, -1.56101807580072, 0.641351538057563];
        let bcs: [f32; 3] = [0.0200833655642113, 0.0401667311284225, 0.0200833655642113];
        filter.init(&acs, &bcs);

        let mut filter2 = DFOneBiQuad::<f32>::create(&filter.coeffs.acs, &filter.coeffs.bcs);

        //filter.process(&mut buffer);
        process_iir_cascade(&mut [&mut filter, &mut filter2], & buffer, &mut output);

        for i in 0..buffer.len() {
            println!("{}", output[i]);
        }
}
