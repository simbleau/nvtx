use nvtx_rs::{range_pop, range_push};
use std::thread::sleep;
use std::time::Duration;

/// Push a range on the NVTX layer which gets popped after an expensive operation. </br>
/// This program should be ran from a profiling application like NVIDIA Nsight Systems!
fn main() {
    range_push("Hello, World!");
    sleep(Duration::from_secs(1)); // Expensive operation here
    range_pop();
}
