use std::io::Write;
use std::str::FromStr;

use config::Config;
mod config;

fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
    match s.find(separator) {
        None => None,
        Some(index) => {
            match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) {
                (Ok(l), Ok(r)) => Some((l, r)),
                _ => None
            }
        }
    }
}


/// parse arg as string
fn split_arg(s: &str) -> Option<Vec<String>> {
    match parse_pair::<String>(&s, ',') {
        Some((w, r)) => Some(vec![w, r]),
        None => None
    }
}

fn main() {
    let mut params = Vec::new();

    let mut args: Vec<String> = std::env::args().skip(1).collect();
    if args.len() == 1 {
        // handle single string input like: 15,5
        if let Some(v) = split_arg(&args[0]) {
            args = v;
        }
    }
    for arg in args {
        // String -> &str
        params.push(f64::from_str(&arg)
            .expect("Error: Unknown argument"));
    }

    if params.len() > 2 {
        writeln!(std::io::stderr(),
                 "Usage: 20min n (working time) n (time for rest)").unwrap();
        std::process::exit(1);
    }

    let (work_t, rest_t) = Config { params: params }.to_tuple();
    println!("work_t {:.1}, rest_t {:.1}", work_t, rest_t);
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
        assert_eq!(parse_pair::<String>("0,5", ','),
            Some(("0".to_string(), "5".to_string())));
        assert_eq!(parse_pair::<String>("0.5,7.8", ','),
            Some(("0.5".to_string(), "7.8".to_string())));
    }

    #[test]
    fn test_split_arg() {
        assert_eq!(split_arg(""), None);
        assert_eq!(split_arg("10,"),
            Some(vec!["10".to_string(), "".to_string()]));
        assert_eq!(split_arg(",10"),
            Some(vec!["".to_string(), "10".to_string()]));
        assert_eq!(split_arg("10,20"),
            Some(vec!["10".to_string(), "20".to_string()]));
        assert_eq!(split_arg("10,20xy"),
            Some(vec!["10".to_string(), "20xy".to_string()]));
        assert_eq!(split_arg("0.5,x"),
            Some(vec!["0.5".to_string(), "x".to_string()]));

        assert_eq!(split_arg("0.5x"), None);
        assert_eq!(split_arg("0.5x1.5"), None);
    }
}
