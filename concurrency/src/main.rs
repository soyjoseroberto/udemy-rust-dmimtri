#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_must_use)]

use std::thread;
use std::time;

fn main() {

    // Create (spawn) a new thread
    let handle = thread::spawn(|| {
        for _ in 1..10 {
            println!("+");
            thread::sleep(time::Duration::from_millis(500));
        }
    });

    // Main thread of execution
    for _ in 1..10 {
        println!("-");
        thread::sleep(time::Duration::from_millis(300));
    }

    // Wait until the non-main thread is done
    handle.join();
}
