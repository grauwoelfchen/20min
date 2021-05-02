#[cfg(test)]
mod test {
    use std::env;
    use std::process::Command;

    const DEFAULT_CARGO_TARGET_DIR: &str = "./target";

    #[test]
    fn test_run_with_invalid_args_with_comma() {
        let target_dir = env::var("CARGO_TARGET_DIR")
            .unwrap_or_else(|_| DEFAULT_CARGO_TARGET_DIR.to_string());
        let program = format!("{}/debug/20min", target_dir);

        let output = Command::new(&program).arg("foo,bar").output().unwrap();
        assert!(String::from_utf8_lossy(&output.stderr).contains("panicked"));
    }

    #[test]
    fn test_run_with_invalid_args_without_comma() {
        let target_dir = env::var("CARGO_TARGET_DIR")
            .unwrap_or_else(|_| DEFAULT_CARGO_TARGET_DIR.to_string());
        let program = format!("{}/debug/20min", target_dir);

        let output = Command::new(&program).arg("foo").output().unwrap();
        assert!(String::from_utf8_lossy(&output.stderr).contains("panicked"));
    }

    #[test]
    fn test_run_with_valid_args() {
        let mut output;

        let target_dir = env::var("CARGO_TARGET_DIR")
            .unwrap_or_else(|_| DEFAULT_CARGO_TARGET_DIR.to_string());
        let program = format!("{}/debug/20min", target_dir);

        output = Command::new(&program)
            .arg("0.01,0.01") // work: 0.6 secs, rest: 0.6 secs
            .output()
            .unwrap();
        assert_eq!(String::from_utf8_lossy(&output.stderr), "");

        output = Command::new(&program)
            .arg("0.01") // work: 0.6 secs, rest: 0.6 secs
            .arg("0.01")
            .output()
            .unwrap();
        assert_eq!(String::from_utf8_lossy(&output.stderr), "");
    }
}
