/*
  Problem 33: String Compression

  Write a function that performs basic string compression using the counts of repeated characters.
  For example, "aabcccccaaa" becomes "a2b1c5a3".
  If the compressed string is not shorter than the original, return the original string.

  Run the tests for this problem with:
    cargo test --test string_compression_test
*/

pub fn compress(s: &str) -> String {
    if s.is_empty() {
        return "".to_string();
    }

    let mut result = String::new();
    let mut prev = None;
    let mut count = 0;

    for c in s.chars() {
        match prev {
            Some(pc) if pc == c => {
                count += 1;
            }
            Some(pc) => {
                result.push(pc);
                result.push_str(&count.to_string());
                count = 1;
                prev = Some(c);
            }
            None => {
                prev = Some(c);
                count = 1;
            }
        }
    }
    if let Some(pc) = prev {
        result.push(pc);
        result.push_str(&count.to_string());
    }

    if result.len() < s.len() {
        result
    } else {
        s.to_string()
    }
}
