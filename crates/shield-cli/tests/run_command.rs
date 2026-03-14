use std::{collections::BTreeMap, env, fs};

use shield_cli::runner::{run_command, to_decrypted};
use tempfile::tempdir;

#[test]
fn run_injects_env_without_creating_env_file() {
    let temp = tempdir().unwrap();
    let original_dir = env::current_dir().unwrap();
    env::set_current_dir(temp.path()).unwrap();

    let mut secrets = BTreeMap::new();
    secrets.insert("DATABASE_URL".to_string(), "postgres://local".to_string());

    let status = run_command(
        &[
            "sh".to_string(),
            "-c".to_string(),
            "test \"$DATABASE_URL\" = \"postgres://local\" && test ! -f .env".to_string(),
        ],
        to_decrypted(secrets),
    )
    .unwrap();

    assert_eq!(status, 0);
    assert!(!temp.path().join(".env").exists());

    env::set_current_dir(original_dir).unwrap();
    fs::remove_dir_all(temp.path()).ok();
}
