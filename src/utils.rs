use anyhow::{Context, Result};
use std::{io::Write, process::Stdio};

/// Format code with nixfmt in PATH
pub fn nixfmt(code: &str) -> Result<String> {
    let mut child = std::process::Command::new("nixfmt")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .context("expected nixfmt to be in PATH")?;

    let child_stdin = child.stdin.as_mut().context("expected stdin")?;
    child_stdin.write_all(code.as_bytes())?;

    let output = child.wait_with_output()?;
    Ok(String::from_utf8(output.stdout)?)
}
