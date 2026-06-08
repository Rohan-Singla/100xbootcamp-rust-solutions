/*
  Problem 15: Parse Integer with Result

  Write a function that takes a &str and attempts to parse it into an i32.
  Return Ok(value) on success, or Err(String) with a descriptive error message on failure.

  Run the tests for this problem with:
    cargo test --test parse_int_test
*/

pub fn parse_int(s: &str) -> Result<i32, String> {
    match s.parse::<i32>() {

        Ok(val) => Ok(val),
        
        Err(e) => Err(format!("failed to parse string: {}", e)),
    }
}
