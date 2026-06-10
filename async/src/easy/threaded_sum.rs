/*
  Problem 71: Multithreaded Sum

  Write a function that takes a Vec<i32> and splits it into two halves.
  Sum each half in a separate thread using std::thread::spawn and return
  the total sum.

  Run the tests for this problem with:
    cargo test --test threaded_sum_test
*/

use std::thread;

pub fn threaded_sum(v: Vec<i32>) -> i32 {
    let halflen = v.len() / 2;
    let (left, right) = v.split_at(halflen);

    let left_vec = left.to_vec();
    let right_vec = right.to_vec();

    let left_handle = thread::spawn(move || {
        left_vec.iter().sum::<i32>()
    });

    let right_handle = thread::spawn(move || {
        right_vec.iter().sum::<i32>()
    });

    let left_sum = left_handle.join().unwrap();
    let right_sum = right_handle.join().unwrap();

    left_sum + right_sum
}
