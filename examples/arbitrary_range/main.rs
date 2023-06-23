use std::thread::{self, sleep};
use std::time::Duration;

use nvtx::{range_end, range_start};

/// Create an arbitrary concurrency range on the NVTX layer. </br>
/// This program should be ran from a profiling application like NVIDIA Nsight
/// Systems!
fn main() {
    let id = thread::spawn(|| {
        let id = range_start!("My Range");
        sleep(Duration::from_millis(100)); // Expensive operation here
        id
    })
    .join()
    .unwrap();

    range_end!(id);
}
