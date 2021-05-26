/**
 * Wraps common functions used for profiling with the NVIDIA Toolkit Extension (NVTX)
 * To learn more, visit https://docs.nvidia.com/gameworks/index.html#gameworkslibrary/nvtx/nvtx_api.htm
 * For the function documentation of wrapped functions, visit the included header below.
 */
#include "NVTX/c/include/nvtx3/nvToolsExt.h"

int _range_push(const char* message) {
    return nvtxRangePushA(message);
}

int _range_pop() {
    return nvtxRangePop();
}

void _mark(const char* message) {
    return nvtxMarkA(message);
}