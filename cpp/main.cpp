//#include <stdint.h>
//#include <stdio.h>
#include <stdio.h>
#include <string.h>
//pub unsafe extern fn c_process32(input: *const f32, output: *mut f32, cacs: *const f32, cbcs: *const f32, buffer_length: *const usize)
//int32_t count_substrings(const char* value, const char* substr);
//void c_process32(const float* input, float* output, const float* cacs, const float* cbcs, unsigned int buffer_length);

#include <iostream>

using std::cout;
using std::endl;

/*
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
*/

extern "C" {

	struct sos32 {
		float _as[3];
		float _bs[3];
	};

	struct fstate32 {
		float _xvs[3];
		float _yvs[3];
	};

	struct filter32 {
		sos32 _coeffs;
		fstate32 _state;
	};

	void c_dummy();
	void c_process32(void* filter, const float* input, float* output, unsigned int buffer_length);

	void bork(void*);
}

int main() {

	float* in = new float[512];
	float* out = new float[512];

	memset(in, 0x0, sizeof(float) * 512);

	in[0] = 1.;

	filter32 filter;

	filter._coeffs._bs[0] = 0.0200833655642113;
	filter._coeffs._bs[1] = 0.0401667311284225;
	filter._coeffs._bs[2] = 0.0200833655642113;

	filter._coeffs._as[0] = 1.;
	filter._coeffs._as[1] = -1.56101807580072;
	filter._coeffs._as[2] = 0.641351538057563;

	c_process32(&filter, in, out, 512);

	for (size_t i = 0; i < 512; ++i)
	{
		cout << out[i] << endl;
	}

    return 0;
}
