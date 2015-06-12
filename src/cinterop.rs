use sos::df_one::DFOneBiQuad;
use std::slice;
use ::Filter;
use ::BiQuadFilter;

#[no_mangle]
pub unsafe extern fn filt_process32(filter: *mut DFOneBiQuad<f32>, input: *const f32, output: *mut f32, buffer_length: usize) {

    //let filter: &mut DFOneBiQuad<f32> = &mut *(filter as *mut DFOneBiQuad<f32>);
    let filter: &mut DFOneBiQuad<f32> = &mut *(filter);

    let inp = slice::from_raw_parts(input, buffer_length);
    let mut outp = slice::from_raw_parts_mut(output, buffer_length);

    filter.process(&inp, &mut outp);
}

#[no_mangle]
pub unsafe extern fn filt_init32(filter: *mut DFOneBiQuad<f32>, acs: *const f32, bcs: *const f32) {

    //let filter: &mut DFOneBiQuad<f32> = &mut *(filter as *mut DFOneBiQuad<f32>);
    let filter: &mut DFOneBiQuad<f32> = &mut *(filter);
    let acs: &mut [f32; 3] = &mut *(acs as *mut [f32; 3]);
    let bcs: &mut [f32; 3] = &mut *(bcs as *mut [f32; 3]);

    filter.init(acs, bcs);
    filter.clear_state();
}

#[no_mangle]
pub extern fn filt_peaking32(filter: *mut DFOneBiQuad<f32>, q: f32, gain: f32, f0: f32, fs: f32) {
    unsafe {
        let filter: &mut DFOneBiQuad<f32> = &mut *(filter);
        ::sos::peaking(&mut filter.coeffs, q, gain, f0, fs);
    }
}

#[no_mangle]
pub extern fn filt_create32() -> *mut DFOneBiQuad<f32> {

    let filter = DFOneBiQuad::<f32>{coeffs: ::sos::one::<f32>(), state: ::sos::df_one::nil::<f32>()};
    let mut obj = Box::new(filter);

    // * derefs the Box into a Dramatic, the &mut re-borrows it into a regular
    // reference.  The constraint ensures we coerce the &mut Dramatic into
    // a *mut Dramatic, which "hides" the reference from the borrow checker.
    let ptr: *mut _ = &mut *obj;

    // Forget discards its argument (passed by-move), without trigger its
    // destructor, if it has one.
    ::std::mem::forget(obj);

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
