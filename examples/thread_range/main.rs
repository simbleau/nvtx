use nvtx_rs::{range_pop, range_push};
use std::thread::sleep;
use std::time::Duration;

/// Push and pop a thread range on the NVTX layer. </br>
/// This program should be ran from a profiling application like NVIDIA Nsight
/// Systems!
fn main() {
    range_push("My Range");
    sleep(Duration::from_millis(100)); // Expensive operation here
    range_pop();
}
