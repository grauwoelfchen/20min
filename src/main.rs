use std::io::Write;
use std::str::FromStr;

struct Args(Vec<f64>);

impl Args {
    fn to_tuple(self) -> (f64, f64) {
        // defaults (seconds)
        let work_t = 15.0 * 60.0;
        let rest_t = 5.0 * 60.0;

        let vec = self.0;
        match vec.len() {
             0 => (work_t, rest_t),
             1 => (vec[0], rest_t),
             2 => (vec[0], vec[1]),
             _ => panic!("Error: Too many values"),
        }
    }
}

#[test]
fn test_args_to_tuple() {
    let mut v;

    v = vec![];
    assert_eq!((900.0, 300.0), Args(v).to_tuple());

    v = vec![10.0];
    assert_eq!((10.0, 300.0), Args(v).to_tuple());

    v = vec![20.0, 30.0];
    assert_eq!((20.0, 30.0), Args(v).to_tuple());

    v = vec![30.0, 40.0, 50.0];
    let res = std::panic::catch_unwind(|| Args(v).to_tuple());
    assert!(res.is_err());
}


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

/// parse arg as string
fn split_arg(s: &str) -> Option<Vec<String>> {
    match parse_pair::<String>(&s, ',') {
        Some((w, r)) => Some(vec![w, r]),
        None => None
    }
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

    let (work_t, rest_t) = Args(params).to_tuple();
    println!("work_t {:.1}, rest_t {:.1}", work_t, rest_t);
}
