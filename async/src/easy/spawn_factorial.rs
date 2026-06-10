/*
  Problem 84: Async Join — Spawn Factorial

  Write an async function that spawns a tokio task (tokio::spawn) to compute
   the factorial of 5 (120). Return the JoinHandle's result after awaiting it.

  Run the tests for this problem with:
    cargo test --test spawn_factorial_test
*/

pub async fn spawn_factorial() -> u64 {
    let factorial = tokio::spawn(async move {
        let mut product = 1u64;
        for i in 1..=5 {
            product *= i;
        }
        product
    });

    factorial.await.unwrap()
}
