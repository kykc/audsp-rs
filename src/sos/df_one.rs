
use ::Numeric;
use ::sos::SecondOrderSection;
use ::BiQuadFilter;
use ::Filter;
use std::cmp;
use ::fcast;

#[repr(C)]
pub struct DFOneState<TReal> {
    pub xvs: [TReal; 3],
    pub yvs: [TReal; 3]
}

#[repr(C)]
pub struct DFOneBiQuad<TReal: Numeric> {
    pub coeffs: SecondOrderSection<TReal>,
    pub state: DFOneState<TReal>
}

#[inline]
pub fn nil<TReal: Numeric>() -> DFOneState<TReal> {
    DFOneState {xvs: [TReal::zero(), TReal::zero(), TReal::zero()], yvs: [TReal::zero(), TReal::zero(), TReal::zero()]}
}

impl<TReal: Numeric> BiQuadFilter<TReal> for DFOneBiQuad<TReal> {

    fn init(&mut self, acs: &[TReal; 3], bcs: &[TReal; 3]) {
        self.coeffs.acs = *acs;
        self.coeffs.bcs = *bcs;
    }
}

impl<TReal: Numeric> Filter<TReal> for DFOneBiQuad<TReal> {
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

    fn clear_state(&mut self) {
        for i in 0..3 {
            self.state.xvs[i] = fcast::<TReal>(0.0);
            self.state.yvs[i] = fcast::<TReal>(0.0);
        }
    }
}
