/// Build script to compile the exported functions for NVIDIA® Tools Extension SDK (NVTX).
fn main() {
    // Compile the exported functions for NVIDIA® Tools Extension SDK (NVTX).
    // This will fail if you do not have the shared headers shipped with
    // CUDA Toolkit and Nsight tools such as Nsight Graphics and Nsight Systems.
    cc::Build::new().file("nvtx-sys/export.c").compile("nvtx");

    // Recompile only if the exported function definitions change.
    println!("cargo:rerun-if-changed=src/nvtx.c");
}
