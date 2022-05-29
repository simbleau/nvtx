/// Build script to compile the exported functions for NVIDIA® Tools Extension SDK (NVTX).
fn main() {
    // Compile the exported functions for NVIDIA® Tools Extension SDK (NVTX).
    // Ensure submodules are initialized.
    cc::Build::new().file("nvtx-sys/export.c").compile("nvtx");

    // Recompile only if the exported function definitions change.
    println!("cargo:rerun-if-changed=nvtx-sys/export.c");
}
