#[cfg(test)]
mod integration_test {
    use std::process::Command;

    // args

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

    // opts

    #[test]
    fn test_run_with_invalid_opts() {
        let mut output;

        output = Command::new("./target/debug/20min")
            .arg("--work")
            .arg("foo")
            .output()
            .unwrap();
        assert!(String::from_utf8_lossy(&output.stderr).contains("panicked"));

        output = Command::new("./target/debug/20min")
            .arg("--rest")
            .arg("foo")
            .output()
            .unwrap();
        assert!(String::from_utf8_lossy(&output.stderr).contains("panicked"));

        output = Command::new("./target/debug/20min")
            .arg("--work")
            .arg("3")
            .arg("--rest")
            .arg("foo")
            .output()
            .unwrap();
        assert!(String::from_utf8_lossy(&output.stderr).contains("panicked"));

        output = Command::new("./target/debug/20min")
            .arg("--work")
            .arg("foo")
            .arg("--rest")
            .arg("3")
            .output()
            .unwrap();
        assert!(String::from_utf8_lossy(&output.stderr).contains("panicked"));

        output = Command::new("./target/debug/20min")
            .arg("--work")
            .arg("foo")
            .arg("--rest")
            .arg("bar")
            .output()
            .unwrap();
        assert!(String::from_utf8_lossy(&output.stderr).contains("panicked"));
    }

    #[test]
    fn test_run_with_unknown_opts() {
        let output = Command::new("./target/debug/20min")
            .arg("--no-more-work")
            .output()
            .unwrap();
        assert!(String::from_utf8_lossy(&output.stderr).contains("panicked"));
    }

    #[test]
    fn test_run_with_valid_opts() {
        let mut output;

        output = Command::new("./target/debug/20min")
            .arg("--work")
            .arg("0.01")
            .arg("--rest")
            .arg("0.01")
            .output()
            .unwrap();
        assert_eq!(String::from_utf8_lossy(&output.stderr), "");

        output = Command::new("./target/debug/20min")
            .arg("-w")
            .arg("0.01")
            .arg("-r")
            .arg("0.01")
            .output()
            .unwrap();
        assert_eq!(String::from_utf8_lossy(&output.stderr), "");
    }
}
