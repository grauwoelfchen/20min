//! A collection as module which have functions that handle the progress.
use std::io::{self, Write};
use std::{thread, time};

// TODO: termcolor support
// https://github.com/rust-lang/cargo/blob/master/src/cargo/core/shell.rs

/// Prints current progress as bar
///
/// # Examples
///
/// ```
/// assert_eq!("", progress(10, 100, "Working"));
/// ```
pub fn progress(count: u64, total: u64, prefix_text: &str) {
  let bar_len = 62; // 80 - (12 + 6)
  let text_width = 12;
  let filled_len = (bar_len * count) / total;
  let value = (100 * count) / total;
  let mut bar = "=".repeat(filled_len as usize);
  bar.push_str(&"-".repeat((bar_len - filled_len) as usize));

  print!(
    "{text:>text_width$} [{bar}] {value}%\r",
    text = &prefix_text,
    text_width = text_width,
    bar = bar,
    value = value
  );
  io::stdout().flush().unwrap();
}

pub fn tick(total: u64, prefix_text: &str) {
  if total > 0 {
    progress(0, total, prefix_text);
    for i in 1..=total {
      thread::sleep(time::Duration::new(1, 0));
      progress(i, total, prefix_text);
    }
    println!();
  }
}
