use std::collections::BTreeMap;
use std::process::Command;

use anyhow::{bail, Context, Result};

use crate::state::DecryptedSecrets;

pub fn run_command(command: &[String], secrets: DecryptedSecrets) -> Result<i32> {
    if command.is_empty() {
        bail!("no command provided");
    }

    let mut process = Command::new(&command[0]);
    if command.len() > 1 {
        process.args(&command[1..]);
    }
    for (key, value) in &secrets.values {
        process.env(key, value);
    }
    let status = process.status().context("failed to start child process")?;
    Ok(status.code().unwrap_or(1))
}

pub fn to_decrypted(values: BTreeMap<String, String>) -> DecryptedSecrets {
    DecryptedSecrets::new(values)
}
