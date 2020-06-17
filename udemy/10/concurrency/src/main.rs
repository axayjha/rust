#![allow(dead_code)]
#![allow(unused_imports)]
#[allow(unused_must_use)]

use std::thread;
use std::time;

// spawning and joining threads

fn main() {
    // println!("Hello, world!");

    let handle = thread::spawn(|| {
        for _ in 1..10 {
            print!("+");
            thread::sleep(time::Duration::from_millis(500));
        }
    });

    for _ in 1..10 {
        print!("-");
        thread::sleep(time::Duration::from_millis(300));
    }

    handle.join();


}
