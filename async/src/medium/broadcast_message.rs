/*
  Problem 89: Message Passing — Broadcast Channel

  Write an async function broadcast_demo() that creates a
  tokio::sync::broadcast channel. Spawn 3 tasks that each subscribe to the
  channel and receive a message. The main function should send a message
  and ensure all tasks receive it. Return the sum of received values.

  Run the tests for this problem with:
    cargo test --test broadcast_message_test
*/

use tokio::sync::broadcast;


pub async fn broadcast_demo() -> i32 {
    let (tx, _) = broadcast::channel(3);

    let mut rx1 = tx.subscribe();
    let mut rx2 = tx.subscribe();
    let mut rx3 = tx.subscribe();

    let h1 = tokio::spawn(async move {
        rx1.recv().await.unwrap()
    });

    let h2 = tokio::spawn(async move {
        rx2.recv().await.unwrap()
    });

    let h3 = tokio::spawn(async move {
        rx3.recv().await.unwrap()
    });

    tx.send(10).unwrap();

    let v1 = h1.await.unwrap();
    let v2 = h2.await.unwrap();
    let v3 = h3.await.unwrap();

    v1 + v2 + v3
}