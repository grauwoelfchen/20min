#[cfg(test)]
mod test {
    use std::process::Command;

    #[test]
    fn test_run_with_invalid_args_with_comma() {
        let output = Command::new("./target/debug/20min")
            .arg("foo,bar")
            .output()
            .unwrap();
        assert!(String::from_utf8_lossy(&output.stderr).contains("panicked"));
    }

    #[test]
    fn test_run_with_invalid_args_without_comma() {
        let output = Command::new("./target/debug/20min")
            .arg("foo")
            .output()
            .unwrap();
        assert!(String::from_utf8_lossy(&output.stderr).contains("panicked"));
    }

    #[test]
    fn test_run_with_valid_args() {
        let mut output;

        output = Command::new("./target/debug/20min")
            .arg("0.01,0.01") // work: 0.6 secs, rest: 0.6 secs
            .output()
            .unwrap();
        assert_eq!(String::from_utf8_lossy(&output.stderr), "");

        output = Command::new("./target/debug/20min")
            .arg("0.01") // work: 0.6 secs, rest: 0.6 secs
            .arg("0.01")
            .output()
            .unwrap();
        assert_eq!(String::from_utf8_lossy(&output.stderr), "");
    }
}
