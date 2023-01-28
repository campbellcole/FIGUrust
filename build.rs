use std::{process::Command, time::SystemTime};

use chrono::{NaiveDate, Utc};
const UNKNOWN_GIT_HASH: &str = "UNKNOWN";

fn main() {
    let git_hash = Command::new("git")
        .args(&["rev-parse", "--short", "HEAD"])
        .output()
        .map(|output| {
            String::from_utf8(output.stdout).unwrap_or_else(|_| UNKNOWN_GIT_HASH.to_string())
        })
        .unwrap_or_else(|_| UNKNOWN_GIT_HASH.to_string());
    let build_date = Utc::now().format("%H:%M:%S %d-%m-%Y").to_string();

    println!("cargo:rustc-env=BUILD_DATE={}", build_date);
    println!("cargo:rustc-env=GIT_HASH={}", git_hash);
}
