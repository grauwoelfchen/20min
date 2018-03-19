#[cfg(test)]
mod integration_test {
    use std::process::Command;

    #[test]
    fn test_run_with_valid_args() {
        let output = Command::new("./target/debug/20min")
            .arg("0.01,0.01")  // work_t: 0.6 secs, rest_t: 0.6 secs
            .output().unwrap();
        assert_eq!(String::from_utf8_lossy(&output.stderr), "");
    }

    #[test]
    fn test_run_with_invalid_work_t() {
        let output = Command::new("./target/debug/20min")
            .arg("foo")
            .output()
            .unwrap();
        assert!(String::from_utf8_lossy(&output.stderr).contains("panicked"));
    }
}
