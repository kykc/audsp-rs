#include <stdio.h>
#include <string.h>

#include <iostream>
#include <array>

using std::cout;
using std::endl;
using std::array;

extern "C" {

	void filt_process32(void* filter, const float* input, float* output, unsigned int buffer_length);
	void filt_init32(void* filter, const float* as, const float* bs);
}

int main() {

	float* in = new float[512];
	float* out = new float[512];

	memset(in, 0x0, sizeof(float) * 512);

	in[0] = 1.;

	char* filter = new char[sizeof(float) * (6 + 6)];

	array<float, 3> as = {1., -1.56101807580072, 0.641351538057563};
	array<float, 3> bs = {0.0200833655642113, 0.0401667311284225, 0.0200833655642113};

	filt_init32(filter, (const float*)&as, (const float*)&bs);
	filt_process32(filter, in, out, 512);

	for (size_t i = 0; i < 512; ++i)
	{
		cout << out[i] << endl;
	}

    return 0;
}
