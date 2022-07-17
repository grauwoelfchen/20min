#[cfg(test)]
mod test {
    use crate::run;

    #[test]
    fn test_run_with_invalid_args_and_comma() {
        let output = run(&["foo,bar"]).expect("command returns output");
        assert!(String::from_utf8_lossy(&output.stderr).contains("panicked"));
    }

    #[test]
    fn test_run_with_invalid_args_no_comma() {
        let output = run(&["foo"]).expect("command returns output");
        assert!(String::from_utf8_lossy(&output.stderr).contains("panicked"));
    }

    #[test]
    fn test_run_with_valid_args_and_comma() {
        // work: 0.6 secs, rest: 0.6 secs
        let output = run(&["0.01,0.01"]).expect("command returns output");
        assert_eq!(String::from_utf8_lossy(&output.stderr), "");
    }

    #[test]
    fn test_run_with_valid_args_no_comma() {
        // work: 0.6 secs, rest: 0.6 secs
        let output = run(&["0.01", "0.01"]).expect("command returns output");
        assert_eq!(String::from_utf8_lossy(&output.stderr), "");
    }
}
