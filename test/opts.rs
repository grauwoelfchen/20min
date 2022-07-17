#[cfg(test)]
mod test {
    use crate::run;

    #[test]
    fn test_run_with_invalid_opts() {
        let output = run(&["--work", "foo"]).unwrap();
        assert!(String::from_utf8_lossy(&output.stderr).contains("panicked"));

        let output = run(&["--rest", "foo"]).unwrap();
        assert!(String::from_utf8_lossy(&output.stderr).contains("panicked"));

        let output = run(&["--work", "3", "--rest", "foo"]).unwrap();
        assert!(String::from_utf8_lossy(&output.stderr).contains("panicked"));

        let output = run(&["--work", "foo", "--rest", "3"]).unwrap();
        assert!(String::from_utf8_lossy(&output.stderr).contains("panicked"));

        let output = run(&["--work", "foo", "--rest", "bar"]).unwrap();
        assert!(String::from_utf8_lossy(&output.stderr).contains("panicked"));
    }

    #[test]
    fn test_run_with_unknown_opts() {
        let output = run(&["--no-more-work"]).unwrap();
        assert!(String::from_utf8_lossy(&output.stderr).contains("panicked"));
    }

    #[test]
    fn test_run_with_valid_opts() {
        let output = run(&["--work", "0.01", "--rest", "0.01"]).unwrap();
        assert_eq!(String::from_utf8_lossy(&output.stderr), "");

        let output = run(&["-w", "0.01", "-r", "0.01"]).unwrap();
        assert_eq!(String::from_utf8_lossy(&output.stderr), "");
    }
}
