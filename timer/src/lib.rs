use std::{
    process::Command,
    time::{Duration, Instant},
};

use anyhow::Result;

pub struct Report {
    pub stdout: String,
    pub stderr: String,
    pub elapsed: Duration,
}

pub fn time(program: &str, args: &[String]) -> Result<Report> {
    let mut cmd = Command::new(program);
    cmd.args(args);
    let start = Instant::now();
    let out = cmd.output()?;
    let stdout = String::from_utf8_lossy(&out.stdout).to_string();

    let stderr = String::from_utf8_lossy(&out.stderr).to_string();
    Ok(Report {
        stdout,
        stderr,
        elapsed: start.elapsed(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn time_times_echo_cmd() {
        let rep = time("echo", &["hey".into()]).unwrap();
        assert_eq!(rep.stdout.trim_end(), "hey", "wrong stdout");
        assert_eq!(rep.stderr.trim_end(), "", "wrong stderr");
        assert!(!rep.elapsed.is_zero(), "zero elapsed time");
    }
}
