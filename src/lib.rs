use std::str::FromStr;

pub mod config;
pub mod ticker;

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
        }
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
///     Some(vec!["10".to_string(), "".to_string()]),
///     split_arg("10,")
/// );
/// ```
pub fn split_arg(s: &str) -> Option<Vec<String>> {
    parse_pair::<String>(s, ',').map(|(w, r)| vec![w, r])
}

/// Outputs the formatted result.
pub fn report(duration: u64) {
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

#[cfg(test)]
mod test {
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
