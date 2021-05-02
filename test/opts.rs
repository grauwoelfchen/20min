#[cfg(test)]
mod test {
    use std::env;
    use std::process::Command;

    const DEFAULT_CARGO_TARGET_DIR: &str = "./target";

    #[test]
    fn test_run_with_invalid_opts() {
        let mut output;

        let target_dir = env::var("CARGO_TARGET_DIR")
            .unwrap_or_else(|_| DEFAULT_CARGO_TARGET_DIR.to_string());
        let program = format!("{}/debug/20min", target_dir);

        output = Command::new(&program)
            .arg("--work")
            .arg("foo")
            .output()
            .unwrap();
        assert!(String::from_utf8_lossy(&output.stderr).contains("panicked"));

        output = Command::new(&program)
            .arg("--rest")
            .arg("foo")
            .output()
            .unwrap();
        assert!(String::from_utf8_lossy(&output.stderr).contains("panicked"));

        output = Command::new(&program)
            .arg("--work")
            .arg("3")
            .arg("--rest")
            .arg("foo")
            .output()
            .unwrap();
        assert!(String::from_utf8_lossy(&output.stderr).contains("panicked"));

        output = Command::new(&program)
            .arg("--work")
            .arg("foo")
            .arg("--rest")
            .arg("3")
            .output()
            .unwrap();
        assert!(String::from_utf8_lossy(&output.stderr).contains("panicked"));

        output = Command::new(&program)
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
        let target_dir = env::var("CARGO_TARGET_DIR")
            .unwrap_or_else(|_| DEFAULT_CARGO_TARGET_DIR.to_string());
        let program = format!("{}/debug/20min", target_dir);

        let output = Command::new(&program)
            .arg("--no-more-work")
            .output()
            .unwrap();
        assert!(String::from_utf8_lossy(&output.stderr).contains("panicked"));
    }

    #[test]
    fn test_run_with_valid_opts() {
        let mut output;

        let target_dir = env::var("CARGO_TARGET_DIR")
            .unwrap_or_else(|_| DEFAULT_CARGO_TARGET_DIR.to_string());
        let program = format!("{}/debug/20min", target_dir);

        output = Command::new(&program)
            .arg("--work")
            .arg("0.01")
            .arg("--rest")
            .arg("0.01")
            .output()
            .unwrap();
        assert_eq!(String::from_utf8_lossy(&output.stderr), "");

        output = Command::new(&program)
            .arg("-w")
            .arg("0.01")
            .arg("-r")
            .arg("0.01")
            .output()
            .unwrap();
        assert_eq!(String::from_utf8_lossy(&output.stderr), "");
    }
}
