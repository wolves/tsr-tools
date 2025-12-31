use std::{
    path::{Path, PathBuf},
    process::{Command, Output},
};

use anyhow::Result;
use walkdir::WalkDir;

#[derive(Default)]
pub struct Slimmer {
    pub dry_run: bool,
}

impl Slimmer {
    pub fn new() -> Self {
        Slimmer::default()
    }

    pub fn slim(&self, path: impl AsRef<Path>) -> Result<String> {
        let mut output = String::new();
        for target in manifests(path)? {
            let mut cmd = self.cargo_clean_cmd(&target);
            let cmd_output = cmd.output()?;
            output.push_str(&summary(target, &cmd_output));
        }
        Ok(output)
    }
    fn cargo_clean_cmd(&self, path: impl AsRef<Path>) -> Command {
        let mut cmd = Command::new("cargo");
        cmd.args([
            "clean",
            "--manifest-path",
            &path.as_ref().to_string_lossy(),
        ]);

        if self.dry_run {
            cmd.arg("--dry-run");
        }

        cmd
    }
}

fn manifests(path: impl AsRef<Path>) -> Result<Vec<PathBuf>> {
    let mut targets = Vec::new();

    for entry in WalkDir::new(path)
        .into_iter()
        .filter_entry(|e| !e.path().ends_with("target/package"))
    {
        let entry = entry?;
        if entry.file_name() == "Cargo.toml" {
            targets.push(entry.path().to_path_buf());
        }
    }

    Ok(targets)
}

fn summary(target: impl AsRef<Path>, output: &Output) -> String {
    format!(
        "{}: {}",
        target.as_ref().parent().unwrap().display(),
        String::from_utf8_lossy(&output.stderr).trim_start(),
    )
}

#[cfg(test)]
mod tests {
    use std::process::{Command, ExitStatus};

    use super::*;

    #[test]
    fn manifests_fn_returns_cargo_toml_paths() {
        let mut actual = manifests("tests/data").unwrap();
        actual.sort();
        let expected: Vec<PathBuf> = vec![
            PathBuf::from("tests/data/proj_1/Cargo.toml"),
            PathBuf::from("tests/data/proj_2/Cargo.toml"),
            PathBuf::from("tests/data/proj_3/Cargo.toml"),
        ];

        assert_eq!(actual, expected, "wrong path manifests");
    }

    #[test]
    fn cargo_clean_cmd_fn_returns_correct_cargo_command() {
        let mut slimmer = Slimmer::new();
        let cmd =
            slimmer.cargo_clean_cmd("tests/data/proj_1/Cargo.toml");
        assert_eq!(
            cmd.get_program(),
            "cargo",
            "wrong cargo clean cmd program"
        );
        assert_eq!(
            cmd.get_args().collect::<Vec<_>>(),
            [
                "clean",
                "--manifest-path",
                "tests/data/proj_1/Cargo.toml"
            ],
            "wrong cargo clean cmd args"
        );
    }

    #[test]
    fn cargo_clean_cmd_fn_honours_dry_run_mode() {
        let mut slimmer = Slimmer::new();
        slimmer.dry_run = true;
        let cmd =
            slimmer.cargo_clean_cmd("tests/data/proj_1/Cargo.toml");
        assert_eq!(
            cmd.get_program(),
            "cargo",
            "wrong cargo clean cmd program"
        );
        assert_eq!(
            cmd.get_args().collect::<Vec<_>>(),
            [
                "clean",
                "--manifest-path",
                "tests/data/proj_1/Cargo.toml",
                "--dry-run",
            ],
            "wrong cargo clean cmd args"
        );
    }

    #[test]
    fn summary_fn_reports_target_path_and_cmd_output() {
        let actual = summary(
            PathBuf::from("./target/Cargo.toml"),
            &Output {
                status: ExitStatus::default(),
                stdout: Vec::new(),
                stderr: String::from(
                    "     Removed 2 files, 1.6MiB total\n",
                )
                .into_bytes(),
            },
        );
        assert_eq!(
            actual, "./target: Removed 2 files, 1.6MiB total\n",
            "wrong formatting"
        );
    }
}
