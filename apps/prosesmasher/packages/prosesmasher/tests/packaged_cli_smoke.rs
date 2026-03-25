#![allow(clippy::disallowed_methods, clippy::panic)]

use prosesmasher_adapters_inbound_cli as _;
use prosesmasher_assertions::packaged_cli_smoke::{
    cargo_bin,
    fixture_path,
    workspace_root,
};

use std::process::Command;

#[test]
#[allow(clippy::panic)]
fn packaged_fs_crate_contains_preset_assets() {
    let output = Command::new(cargo_bin())
        .current_dir(workspace_root())
        .args([
            "package",
            "--list",
            "-p",
            "prosesmasher-adapters-outbound-fs-runtime",
        ])
        .output()
        .unwrap_or_else(|e| panic!("cargo package --list failed to start: {e}"));

    assert!(output.status.success(), "cargo package --list failed");
    let stdout = String::from_utf8(output.stdout).unwrap_or_else(|e| panic!("stdout utf8: {e}"));
    assert!(
        stdout.contains("presets/article-en.json"),
        "article preset missing from package list"
    );
    assert!(
        stdout.contains("presets/full-config-en.json"),
        "full config missing from package list"
    );
}

#[test]
#[allow(clippy::panic)]
fn wrapper_binary_smoke_uses_preset_assets() {
    let dump = Command::new(cargo_bin())
        .current_dir(workspace_root())
        .args([
            "run",
            "-q",
            "-p",
            "prosesmasher",
            "--",
            "dump-config",
            "--preset",
            "article-en",
        ])
        .output()
        .unwrap_or_else(|e| panic!("dump-config process failed: {e}"));

    assert!(dump.status.success(), "dump-config should succeed");
    let dump_stdout =
        String::from_utf8(dump.stdout).unwrap_or_else(|e| panic!("dump stdout utf8: {e}"));
    assert!(
        dump_stdout.contains("\"locale\": \"en\""),
        "dump output should look like JSON config"
    );

    let check = Command::new(cargo_bin())
        .current_dir(workspace_root())
        .args([
            "run",
            "-q",
            "-p",
            "prosesmasher",
            "--",
            "check",
            fixture_path()
                .to_str()
                .unwrap_or_else(|| panic!("fixture path utf8")),
            "--preset",
            "article-en",
            "--format",
            "json",
        ])
        .output()
        .unwrap_or_else(|e| panic!("check process failed: {e}"));

    assert_eq!(
        check.status.code(),
        Some(1),
        "failing content should exit with lint code 1"
    );
    assert!(
        check.stderr.is_empty(),
        "json mode should keep stderr empty for check failures"
    );
    let stdout =
        String::from_utf8(check.stdout).unwrap_or_else(|e| panic!("check stdout utf8: {e}"));
    assert!(
        stdout.contains("\"schema_version\": 1"),
        "json output should include schema version"
    );
    assert!(
        stdout.contains("\"exit_reason\": \"check-failures\""),
        "json output should include exit reason"
    );
}
