extern crate num;
//extern crate libc;

use num::Num;
use num::Zero;
use std::cmp;
use std::slice;
use std::ptr;
use std::boxed;

#[repr(u8)]
pub enum c_void {
    __variant1,
    __variant2,
}

/*extern {
    fn calloc(nitems: usize, size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void) -> ();
}*/

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
pub unsafe extern fn filt_process32(filter: *mut c_void, input: *const f32, output: *mut f32, buffer_length: usize) {

    let filter: &mut DFOneBiQuad<f32> = &mut *(filter as *mut DFOneBiQuad<f32>);

    let inp = slice::from_raw_parts(input, buffer_length);
    let mut outp = slice::from_raw_parts_mut(output, buffer_length);

    filter.process(&inp, &mut outp);
}

#[no_mangle]
pub unsafe extern fn filt_init32(filter: *mut c_void, acs: *const f32, bcs: *const f32) {

    let filter: &mut DFOneBiQuad<f32> = &mut *(filter as *mut DFOneBiQuad<f32>);

    for i in 0..3 {
        filter.coeffs.acs[i] = *acs.offset(i as isize);
        filter.coeffs.bcs[i] = *bcs.offset(i as isize);
        filter.state.xvs[i] = 0.0;
        filter.state.yvs[i] = 0.0;
    }
}

#[no_mangle]
pub extern fn filt_create32() -> *mut DFOneBiQuad<f32> {

    let mut obj = Box::new(DFOneBiQuad::<f32>::one());

    // * derefs the Box into a Dramatic, the &mut re-borrows it into a regular
    // reference.  The constraint ensures we coerce the &mut Dramatic into
    // a *mut Dramatic, which "hides" the reference from the borrow checker.
    let ptr: *mut _ = &mut *obj;

    // Forget discards its argument (passed by-move), without trigger its
    // destructor, if it has one.
    unsafe { std::mem::forget(obj);}

    ptr
}

#[no_mangle]
pub extern "C" fn filt_destroy32(ptr: *mut DFOneBiQuad<f32>) {
    // For is_null:
    //use std::ptr::PtrExt;

    // First, we **must** check to see if the pointer is null.
    if ptr.is_null() {
        // Do nothing.
        return;
    }

    // Now, we know the pointer is non-null, we can continue.
    let obj: Box<DFOneBiQuad<f32>> = unsafe { ::std::mem::transmute(ptr) };

    // We don't *have* to do anything else; once obj goes out of scope, it will
    // be dropped.  I'm going to drop it explicitly, however, for clarity.
    ::std::mem::drop(obj);
}
