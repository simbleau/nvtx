use std::thread::sleep;
use std::time::Duration;

use nvtx::{range_pop, range_push};

/// Push and pop a thread range on the NVTX layer. </br>
/// This program should be ran from a profiling application like NVIDIA Nsight
/// Systems!
fn main() {
    range_push!("My Range");
    sleep(Duration::from_millis(100)); // Expensive operation here
    range_pop!();
}
