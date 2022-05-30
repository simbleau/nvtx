use std::thread::{self};
use std::time::Duration;

use nvtx::name_thread;

/// Annotate threads with names. </br> This program should be ran from a
/// profiling application like NVIDIA Nsight Systems!
fn main() {
    name_thread!("Main Thread");
    let handler2 = thread::spawn(|| {
        name_thread!("Thread 2");
        thread::sleep(Duration::from_secs_f32(0.2));
    });
    let handler3 = thread::spawn(|| {
        name_thread!("Thread 3");
        thread::sleep(Duration::from_secs_f32(0.3));
    });
    thread::sleep(Duration::from_secs_f32(0.5));
    handler2.join().unwrap();
    handler3.join().unwrap();
}
