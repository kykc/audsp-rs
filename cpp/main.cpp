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
	void* filt_create32();
	void filt_destroy32(void*);
}

struct BiQuad32
{
private:
	void* _handle;
	// Non-copyable
	BiQuad32(const BiQuad32&) {}
public:

	BiQuad32()
	{
		_handle = filt_create32();
	}

	void init(const float* as, const float* bs)
	{
		filt_init32(_handle, as, bs);
	}

	void processBlock(const float* input, float* output, unsigned int buffer_length)
	{
		filt_process32(_handle, input, output, buffer_length);
	}

	~BiQuad32()
	{
		filt_destroy32(_handle);
	}
};

int main() {

	float* in = new float[512];
	float* out = new float[512];

	memset(in, 0x0, sizeof(float) * 512);

	in[0] = 1.;

	array<float, 3> as = {1., -1.56101807580072, 0.641351538057563};
	array<float, 3> bs = {0.0200833655642113, 0.0401667311284225, 0.0200833655642113};

	BiQuad32 filter;
	filter.init((const float*)&as, (const float*)&bs);
	filter.processBlock(in, out, 512);

	for (size_t i = 0; i < 512; ++i)
	{
		cout << out[i] << endl;
	}

    return 0;
}
