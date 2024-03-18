use std::process::{Command, Stdio};

use crate::provider::Provider;

pub struct OutputGenerator {
    code_provider: Box<dyn Provider>,
    input_provider: Box<dyn Provider>,
}

impl OutputGenerator {
    pub fn invoke(&self) -> String {
        let code = self.code_provider.get();
        let input = self.input_provider.get();
        let mut echo = Command::new("echo")
            .arg(input)
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();
        let stdin = echo.stdout.take().unwrap();

        let awk = Command::new("awk")
            .arg(code)
            .stdin(stdin)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap()
            .wait_with_output()
            .unwrap();

        // Stderr and Stdout should never both be empty (or both full)
        if awk.stderr.is_empty() {
            String::from_utf8_lossy(&awk.stdout).to_string()
        } else {
            String::from_utf8_lossy(&awk.stderr).to_string()
        }
    }
}
