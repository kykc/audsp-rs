extern crate num;
//extern crate libc;

use num::Num;
use num::Zero;
use std::cmp;
use std::slice;

#[repr(u8)]
pub enum c_void {
    __variant1,
    __variant2,
}

#[repr(C)]
pub struct SecondOrderSection<TReal> {
    pub acs: [TReal; 3],
    pub bcs: [TReal; 3]
}

#[repr(C)]
pub struct DFOneState<TReal> {
    pub xvs: [TReal; 3],
    pub yvs: [TReal; 3]
}

#[repr(C)]
pub struct DFOneBiQuad<TReal: Num + Copy> {
    pub coeffs: SecondOrderSection<TReal>,
    pub state: DFOneState<TReal>
}

pub trait BiQuadFilter<TReal: Num + Copy> {
    fn process(&mut self, input: & [TReal], output: &mut[TReal]);
    fn init(&mut self, acs: &[TReal; 3], bcs: &[TReal; 3]);
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
pub unsafe extern fn bork(data: *mut c_void) {
    let object: &DFOneBiQuad<f32> = &mut *(data as *mut DFOneBiQuad<f32>);
    println!("{}", object.coeffs.acs[0]);
}

#[no_mangle]
pub unsafe extern fn c_process32(filter: *mut c_void, input: *const f32, output: *mut f32, buffer_length: usize) {

    let filter: &mut DFOneBiQuad<f32> = &mut *(filter as *mut DFOneBiQuad<f32>);

    let inp = slice::from_raw_parts(input, buffer_length);
    let mut outp = slice::from_raw_parts_mut(output, buffer_length);

    filter.process(&inp, &mut outp);
}

#[no_mangle]
pub extern fn c_dummy() {
   println!("zhopa");
}
