/**
 * Wraps common functions used for profiling with the NVIDIA Toolkit Extension (NVTX)
 * To learn more, visit https://docs.nvidia.com/gameworks/index.html#gameworkslibrary/nvtx/nvtx_api.htm
 * For the function documentation of wrapped functions, visit the included header below.
 */
#include "NVTX/c/include/nvtx3/nvToolsExt.h"

/* Import threading for Windows */
#if defined(_WIN32) || defined(_WIN64) || defined(__CYGWIN__)
#include <windows.h>
#define THREAD_ID GetCurrentThreadId()
#endif

/* Threading for Android */
#if defined(__ANDROID__)
#include <unistd.h>
#define THREAD_ID gettid()
#endif

/* Threading for Linux */
#if defined(__linux__)
#include <sys/syscall.h>
#define THREAD_ID syscall(SYS_gettid)
#endif

/* Threading for OSX */
#if defined(__APPLE__) || defined(__MACH__) // Apple OSX and iOS (Darwin)
#include <sys/syscall.h>
#define THREAD_ID syscall(SYS_thread_selfid)
#endif

int ffi_range_push(const char *message)
{
    return nvtxRangePushA(message);
}

int ffi_range_pop()
{
    return nvtxRangePop();
}

int ffi_range_start(const char *message)
{
    return nvtxRangeStartA(message);
}

void ffi_range_end(int id)
{
    return nvtxRangeEnd(id);
}

void ffi_mark(const char *message)
{
    return nvtxMarkA(message);
}

void ffi_name_thread(const char *name)
{
    return nvtxNameOsThreadA(THREAD_ID, name);
}
