#[cfg(test)]
mod integration_test {
    use std::process::Command;

    #[test]
    fn test_run_without_any_args() {
        let output = Command::new("./target/debug/20min")
            .output().unwrap();
        assert_eq!(String::from_utf8_lossy(&output.stdout),
                   "work_t 900.0, rest_t 300.0\n");
        assert_eq!(String::from_utf8_lossy(&output.stderr), "");
    }

    #[test]
    fn test_run_with_invalid_work_t() {
        let output = Command::new("./target/debug/20min")
            .arg("foo")
            .output().unwrap();
        assert_eq!(String::from_utf8_lossy(&output.stdout), "");
        assert!(String::from_utf8_lossy(&output.stderr).contains("panicked"));
    }
}
