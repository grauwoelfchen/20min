use std::str::FromStr;

pub struct Config {
  pub params: Vec<u64>,
}

impl Config {
  const PARSE_ERROR: &'static str = "Error: Unknown argument";

  pub fn new(args: Vec<String>) -> Self {
    let mut p: Vec<u64> = Vec::new();
    for arg in args {
      let v: u64;
      // String -> u64
      if arg.contains('.') {
        let f = f64::from_str(&arg).expect(Config::PARSE_ERROR);
        v = (f * 60.0).round() as u64;
      } else {
        v = u64::from_str(&arg).expect(Config::PARSE_ERROR) * 60;
      }
      p.push(v);
    }
    Config { params: p }
  }

  pub fn to_tuple(self) -> (u64, u64) {
    // defaults (seconds)
    let work_t = 15 * 60;
    let rest_t = 5 * 60;

    let vec = self.params;
    match vec.len() {
      0 => (work_t, rest_t),
      1 => (vec[0], rest_t),
      2 => (vec[0], vec[1]),
      _ => panic!("Error: Too many values"),
    }
  }
}

#[cfg(test)]
mod config_test {
  use std::panic;
  use super::*;

  #[test]
  fn test_config_to_tuple() {
    let mut c;

    c = Config { params: vec![] };
    assert_eq!((900, 300), c.to_tuple());

    c = Config {
      params: vec![10],
    };
    assert_eq!((10, 300), c.to_tuple());

    c = Config {
      params: vec![20, 30],
    };
    assert_eq!((20, 30), c.to_tuple());

    c = Config {
      params: vec![305, 45],
    };
    assert_eq!((305, 45), c.to_tuple());

    c = Config {
      params: vec![30, 40, 50],
    };
    let res = panic::catch_unwind(|| c.to_tuple());
    assert!(res.is_err());
  }
}
