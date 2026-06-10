/*
  Problem 74: Shared State — Mutex Counter

  Write a function that spawns 10 threads. Each thread should increment a shared
  counter 100 times. Use Arc<Mutex<i32>> to share the counter safely.
  Wait for all threads to finish and return the final counter value.

  Run the tests for this problem with:
    cargo test --test shared_counter_test
*/

use std::sync::{Arc, Mutex};
use std::thread;

pub fn multithreaded_counter() -> i32 {
    let count = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let countclone = Arc::clone(&count);
        let handle = thread::spawn(move || {
            for _ in 0..100 {
                let mut val = countclone.lock().unwrap();
                *val += 1;
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let final_count = count.lock().unwrap();
    *final_count
}
