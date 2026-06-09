/*
  Problem 19: Word Counter

  Write a function that takes a &str and returns a HashMap<String, usize>
  where each key is a lowercase word and each value is the number of occurrences.
  Split on whitespace and convert to lowercase.

  Run the tests for this problem with:
    cargo test --test word_counter_test
*/

use std::collections::HashMap;

pub fn word_count(text: &str) -> HashMap<String, usize> {
    let mut map: HashMap<String, usize> = HashMap::new();

    for word in text.split_whitespace() {
        let word = word.to_lowercase();

      // if entry doesn't exist put 0 else increment by 1

        *map.entry(word).or_insert(0) += 1;
    }

    return map
}
