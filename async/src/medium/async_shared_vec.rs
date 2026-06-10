/*
  Problem 91: Async Shared State — Mutex Vec

  Rewrite Problem 74 / 79 using tokio::sync::Mutex to share a Vec<i32>.
  Spawn 5 tasks, each pushing 10 numbers into the vector.
  Return the length of the final vector.

  Run the tests for this problem with:
    cargo test --test async_shared_vec_test
*/

use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn async_shared_vec() -> usize {
    let data = Arc::new(Mutex::new(vec![]));
    let mut handles = Vec::new();

    for _ in 0..5 {
        let data = Arc::clone(&data);
        let handle = tokio::spawn(async move {
            for i in 0..10 {
                let mut vec = data.lock().await;
                vec.push(i);
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.await.unwrap();
    }

    let vec = data.lock().await;
    vec.len()
}
