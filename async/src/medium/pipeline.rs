/*
  Problem 77: Message Passing — Data Pipeline

  Create a three-stage pipeline using mpsc channels:
  1. Producer: Sends numbers 1..=5.
  2. Processor: Receives numbers, squares them, and sends to next stage.
  3. Consumer: Receives squared numbers and sums them.
  Implement this using three threads and return the final sum.

  Run the tests for this problem with:
    cargo test --test pipeline_test
*/

use std::sync::mpsc;
use std::thread;

pub fn data_pipeline() -> i32 {
    let (tx1, rx1) = mpsc::channel();
    let (tx2, rx2) = mpsc::channel();

    let producer = thread::spawn(move || {
        for i in 1..=5 {
            let _ = tx1.send(i);
        }
    });

    let processor = thread::spawn(move || {
        for num in rx1 {
            let _ = tx2.send(num * num);
        }
    });

    let consumer = thread::spawn(move || {
        let mut sum = 0;
        for num in rx2 {
            sum += num;
        }
        sum
    });

    producer.join().unwrap();
    processor.join().unwrap();

    consumer.join().unwrap()
}
