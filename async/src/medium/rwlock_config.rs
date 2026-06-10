/*
  Problem 75: Shared State — RwLock Configuration

  Define a Config struct with a HashMap<String, String>. Wrap it in Arc<RwLock<Config>>.
  Write a function that spawns 5 reader threads and 1 writer thread.
  Readers should read a specific key, while the writer updates it.
  Return the final value of the configuration key.

  Run the tests for this problem with:
    cargo test --test rwlock_config_test
*/

use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::thread;

pub struct Config {
    pub settings: HashMap<String, String>,
}

pub fn update_and_read_config() -> String {
    let mut map = HashMap::new();
    map.insert("important".to_string(), "initial".to_string());
    let config = Arc::new(RwLock::new(Config { settings: map }));

    let mut handles = vec![];

    for _ in 0..5 {
        let config_clone = Arc::clone(&config);
        handles.push(thread::spawn(move || {
            // Simulate read
            let guard = config_clone.read().unwrap();
            guard.settings.get("important").cloned()
        }));
    }

    let config_writer = Arc::clone(&config);
    let writer_handle = thread::spawn(move || {
        let mut guard = config_writer.write().unwrap();
        guard.settings.insert("important".to_string(), "updated".to_string());
    });

    writer_handle.join().unwrap();

    for handle in handles {
        let _ = handle.join();
    }

    let guard = config.read().unwrap();
    guard.settings.get("important").cloned().unwrap_or_default()
}
