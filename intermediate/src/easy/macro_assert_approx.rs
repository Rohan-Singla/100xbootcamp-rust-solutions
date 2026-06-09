/*
  Problem 58: Macro — assert_approx_eq!

  Write a declarative macro assert_approx_eq! that takes two f64 expressions
  and an optional epsilon (default 1e-10). It should panic if the values
  differ by more than epsilon.

  Run the tests for this problem with:
    cargo test --test macro_assert_approx_test
*/

#[macro_export]
macro_rules! assert_approx_eq {
    ($a:expr, $b:expr) => {
        $crate::assert_approx_eq!($a, $b, 1e-10)
    };
    ($a:expr, $b:expr, $eps:expr) => {{
        let a_val: f64 = $a;
        let b_val: f64 = $b;
        let eps_val: f64 = $eps;
        if (a_val - b_val).abs() > eps_val {
            panic!(
                "assertion failed: |{} - {}| = {} > {} (eps)\nleft: `{}`\nright: `{}`",
                a_val, b_val, (a_val - b_val).abs(), eps_val, a_val, b_val
            );
        }
    }};
}
