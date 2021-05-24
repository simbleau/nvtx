use std::time::Duration;

use nvtx_rs::{range_pop, range_push};
use std::thread::sleep;

fn main() {
    range_push("Hello, World!");
    sleep(Duration::from_secs(1)); // Expensive operation here
    range_pop();
}
