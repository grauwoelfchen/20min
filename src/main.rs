//! A command line working timer `20min`.
//!
//! # Examples
//!
//! ```zsh
//! % 20min 15,5
//! % 20min 0.5 5
//! ```
extern crate structopt;

use std::str::FromStr;

use structopt::StructOpt;

use config::Config;
mod config;
use ticker::tick;
mod ticker;

/// Command line options.
#[derive(Debug, StructOpt)]
#[structopt(
  name = "20min",
  about = "\nA command line working timer. Run with arguments `15,5` (minutes) \
           or OPTIONS \nlike `--work 15 --rest 15`."
)]
struct Opts {
  /// Working time
  #[structopt(short = "w", long = "work", default_value = "15")]
  work: String,

  /// Time of rest
  #[structopt(short = "r", long = "rest", default_value = "5")]
  rest: String,
}

/// Create new pair using `Option`.
///
/// This is a private utility function to parse command line args from string
/// into pair.
///
/// # Errors
///
/// This function will not return any errors, otherwise returns None.
///
/// # Examples
///
/// ```
/// assert_eq!(None, parse_pair::<i32>("", ','));
/// assert_eq!(Some("15", "5"), parse_pair::<i32>("15,5", ','));
/// ```
fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
  match s.find(separator) {
    None => None,
    Some(index) => {
      match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) {
        (Ok(l), Ok(r)) => Some((l, r)),
        _ => None,
      }
    },
  }
}

/// Parses an arg into Vec.
///
/// # Errors
///
/// This private function will not return any errors, otherwise returns None.
///
/// # Examples
///
/// ```
/// assert_eq!(
///   Some(vec!["10".to_string(), "".to_string()]),
///   split_arg("10,")
/// );
/// ```
fn split_arg(s: &str) -> Option<Vec<String>> {
  match parse_pair::<String>(s, ',') {
    Some((w, r)) => Some(vec![w, r]),
    None => None,
  }
}

/// Outputs the formatted result.
fn report(duration: u64) {
  let prefix = "Finished";
  let prefix_width = 12;
  let info = "work [unoptimized + progressbar]";

  println!(
    "{prefix:>prefix_width$} {info} target(s) in {duration} secs\r",
    prefix = prefix,
    prefix_width = prefix_width,
    info = info,
    duration = duration
  );
}

fn main() {
  let mut args: Vec<String> = std::env::args().skip(1).collect();

  // detect argument type ("20,3" or "--work 5 --rest 0.1")
  let mut use_opts: bool = false;
  let o = [
    "-w",
    "--work",
    "-r",
    "--rest",
    "-h",
    "--help",
    "-V",
    "--version",
  ];
  for (_, opt) in o.iter().enumerate() {
    if args.contains(&(opt.to_string())) {
      use_opts = true;
      break;
    }
  }

  if use_opts {
    let opts = Opts::from_args();
    args = vec![opts.work, opts.rest];
  }

  if args.len() == 1 {
    // handle single string input like: "15,5"
    if let Some(v) = split_arg(&args[0]) {
      args = v;
    }
  }

  let c = Config::new(args);
  if c.params.len() > 2 {
    eprintln!("Usage: 20min n (working time) n (time for rest)");
    std::process::exit(1);
  }

  let (work_t, rest_t) = c.to_tuple();

  tick(work_t, "Working");
  report(work_t);

  tick(rest_t, "Resting");
  report(rest_t);
}

#[cfg(test)]
mod main_test {
  use super::*;

  #[test]
  fn test_parse_pair() {
    assert_eq!(parse_pair::<i32>("", ','), None);
    assert_eq!(parse_pair::<i32>("10,", ','), None);
    assert_eq!(parse_pair::<i32>(",10", ','), None);
    assert_eq!(parse_pair::<i32>("10,20", ','), Some((10, 20)));
    assert_eq!(parse_pair::<i32>("10,20xy", ','), None);
    assert_eq!(parse_pair::<f64>("0.5x", 'x'), None);
    assert_eq!(parse_pair::<f64>("0.5x1.5", 'x'), Some((0.5, 1.5)));
    assert_eq!(
      parse_pair::<String>("0,5", ','),
      Some(("0".to_string(), "5".to_string()))
    );
    assert_eq!(
      parse_pair::<String>("0.5,7.8", ','),
      Some(("0.5".to_string(), "7.8".to_string()))
    );
  }

  #[test]
  fn test_split_arg() {
    assert_eq!(split_arg(""), None);
    assert_eq!(
      split_arg("10,"),
      Some(vec!["10".to_string(), "".to_string()])
    );
    assert_eq!(
      split_arg(",10"),
      Some(vec!["".to_string(), "10".to_string()])
    );
    assert_eq!(
      split_arg("10,20"),
      Some(vec!["10".to_string(), "20".to_string()])
    );
    assert_eq!(
      split_arg("10,20xy"),
      Some(vec!["10".to_string(), "20xy".to_string()])
    );
    assert_eq!(
      split_arg("0.5,x"),
      Some(vec!["0.5".to_string(), "x".to_string()])
    );

    assert_eq!(split_arg("0.5x"), None);
    assert_eq!(split_arg("0.5x1.5"), None);
  }
}
