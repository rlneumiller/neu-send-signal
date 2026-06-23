use assert_cmd::Command;
use predicates::prelude::*;
use std::env;

#[test]
fn strict_mode_rejects_unquoted_multiword() {
    unsafe {
        env::remove_var("CALLMEBOT_MYPHONE");
        env::remove_var("CALLMEBOT_APIKEY");
    }

    let mut cmd = Command::cargo_bin("neu-send-signal").unwrap();

    cmd.arg("Hello").arg("world")
        .assert()
        .failure()
        .stderr(predicate::str::contains("must be provided as a single quoted string"));
}

#[test]
fn strict_mode_accepts_single_argument() {
    unsafe {
        env::remove_var("CALLMEBOT_MYPHONE");
        env::remove_var("CALLMEBOT_APIKEY");
    }

    let mut cmd = Command::cargo_bin("neu-send-signal").unwrap();

    cmd.arg("Hello")
        .assert()
        .failure()
        .stderr(predicate::str::contains("CALLMEBOT_MYPHONE"));
}

#[test]
fn shell_safe_mode_accepts_multiword() {
    unsafe {
        env::remove_var("CALLMEBOT_MYPHONE");
        env::remove_var("CALLMEBOT_APIKEY");
    }

    let mut cmd = Command::cargo_bin("neu-send-signal").unwrap();

    cmd.arg("--shell-safe")
        .arg("Hello")
        .arg("world")
        .assert()
        .failure()
        .stderr(predicate::str::contains("CALLMEBOT_MYPHONE"));
}

#[test]
fn raw_mode_accepts_anything() {
    unsafe {
        env::remove_var("CALLMEBOT_MYPHONE");
        env::remove_var("CALLMEBOT_APIKEY");
    }

    let mut cmd = Command::cargo_bin("neu-send-signal").unwrap();

    cmd.arg("--raw")
        .arg("Hello")
        .arg("world")
        .arg("here's")
        .arg("an")
        .arg("apostrophe")
        .assert()
        .failure()
        .stderr(predicate::str::contains("CALLMEBOT_MYPHONE"));
}
