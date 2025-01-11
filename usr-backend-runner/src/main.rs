use std::process::Stdio;

fn main() {
    loop {
        let result = std::process::Command::new("cargo")
            .args([
                "run",
                "-p",
                "usr-backend",
                #[cfg(not(debug_assertions))]
                "--release"
            ])
            .stderr(Stdio::piped())
            .stdout(Stdio::null())
            .output();
        match result {
            Ok(output) => {
                let _ = std::fs::write("runner.log", format!("{}\n{}", String::from_utf8(output.stderr).unwrap(), output.status));
            }
            Err(e) => {
                let _ = std::fs::write("runner.log", e.to_string());
            }
        }
        std::thread::sleep(std::time::Duration::from_secs(5));
    }
}
