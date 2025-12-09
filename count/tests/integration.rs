use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn binary_with_no_args_prints_usage() {
    Command::cargo_bin("count")
        .unwrap()
        .assert()
        .failure()
        .stderr(predicate::str::contains("Usage"));
}

#[test]
fn binary_counts_lines_in_named_file() {
    let want = "tests/data/test.txt: 2 lines\ntests/data/test2.txt: 3 lines\n";
    Command::cargo_bin("count")
        .unwrap()
        .args(["tests/data/test.txt", "tests/data/test2.txt"])
        .assert()
        .success()
        .stdout(predicate::eq(want));
}

#[test]
fn binary_with_w_flag_counts_words() {
    let want = "tests/data/test.txt: 7 words\n";
    Command::cargo_bin("count")
        .unwrap()
        .args(["-w", "tests/data/test.txt"])
        .assert()
        .success()
        .stdout(predicate::eq(want));
}
