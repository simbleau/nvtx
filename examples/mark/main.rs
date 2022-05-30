use std::thread::sleep;
use std::time::Duration;

use nvtx_rs::mark;

/// Marks an instantaneous event on the NVTX layer. </br>
/// This program should be ran from a profiling application like NVIDIA Nsight
/// Systems!
fn main() {
    mark!("Operation A - Begin");
    sleep(Duration::from_millis(150)); // Expensive operation here
    mark!("Operation B - Begin");
    sleep(Duration::from_millis(75)); // Expensive operation here
}
