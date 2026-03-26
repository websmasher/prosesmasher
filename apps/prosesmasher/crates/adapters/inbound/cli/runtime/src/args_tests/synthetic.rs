use prosesmasher_adapters_inbound_cli_assertions::args::{
    ConfigSource, assert_check_command, assert_dump_config_full_command,
    assert_dump_config_preset_command, assert_list_presets_command, assert_parse_err, parse_ok,
};

#[test]
fn parse_check_with_file() {
    let args = parse_ok(["prosesmasher", "check", "foo.md", "--preset", "general-en"]);
    assert_check_command(
        &args,
        Some("foo.md"),
        false,
        ConfigSource::Preset("general-en"),
        None,
        None,
        "text",
        "failures",
        false,
        "parse check with file",
    );
}

#[test]
fn parse_check_with_config() {
    let args = parse_ok(["prosesmasher", "check", "foo.md", "--config", "c.json"]);
    assert_check_command(
        &args,
        Some("foo.md"),
        false,
        ConfigSource::Config("c.json"),
        None,
        None,
        "text",
        "failures",
        false,
        "parse check with config",
    );
}

#[test]
fn parse_check_with_preset() {
    let args = parse_ok(["prosesmasher", "check", "foo.md", "--preset", "article-en"]);
    assert_check_command(
        &args,
        Some("foo.md"),
        false,
        ConfigSource::Preset("article-en"),
        None,
        None,
        "text",
        "failures",
        false,
        "parse check with preset",
    );
}

#[test]
fn parse_check_with_config_and_preset_fails() {
    assert_parse_err(
        [
            "prosesmasher",
            "check",
            "foo.md",
            "--config",
            "c.json",
            "--preset",
            "article-en",
        ],
        "config and preset should conflict",
    );
}

#[test]
fn parse_check_with_group() {
    let args = parse_ok([
        "prosesmasher",
        "check",
        "foo.md",
        "--preset",
        "general-en",
        "--group",
        "quality",
    ]);
    assert_check_command(
        &args,
        Some("foo.md"),
        false,
        ConfigSource::Preset("general-en"),
        Some("quality"),
        None,
        "text",
        "failures",
        false,
        "parse check with group",
    );
}

#[test]
fn parse_check_requires_config_source() {
    let args = parse_ok(["prosesmasher", "check", "foo.md"]);
    assert_check_command(
        &args,
        Some("foo.md"),
        false,
        ConfigSource::None,
        None,
        None,
        "text",
        "failures",
        false,
        "clap should defer config-source validation",
    );
}

#[test]
fn parse_missing_path_fails() {
    let args = parse_ok(["prosesmasher", "check"]);
    assert_check_command(
        &args,
        None,
        false,
        ConfigSource::None,
        None,
        None,
        "text",
        "failures",
        false,
        "clap should allow runtime to decide whether path is required",
    );
}

#[test]
fn parse_no_subcommand_fails() {
    assert_parse_err(["prosesmasher"], "no subcommand should fail");
}

#[test]
fn parse_check_with_check_filter() {
    let args = parse_ok([
        "prosesmasher",
        "check",
        "foo.md",
        "--preset",
        "general-en",
        "--check",
        "prohibited-terms,em-dashes",
    ]);
    assert_check_command(
        &args,
        Some("foo.md"),
        false,
        ConfigSource::Preset("general-en"),
        None,
        Some("prohibited-terms,em-dashes"),
        "text",
        "failures",
        false,
        "parse check with check filter",
    );
}

#[test]
fn parse_check_with_format_json() {
    let args = parse_ok([
        "prosesmasher",
        "check",
        "foo.md",
        "--preset",
        "general-en",
        "--format",
        "json",
    ]);
    assert_check_command(
        &args,
        Some("foo.md"),
        false,
        ConfigSource::Preset("general-en"),
        None,
        None,
        "json",
        "failures",
        false,
        "parse check with json format",
    );
}

#[test]
fn parse_check_with_include_checks() {
    let args = parse_ok([
        "prosesmasher",
        "check",
        "foo.md",
        "--preset",
        "general-en",
        "--format",
        "json",
        "--include-checks",
    ]);
    assert_check_command(
        &args,
        Some("foo.md"),
        false,
        ConfigSource::Preset("general-en"),
        None,
        None,
        "json",
        "failures",
        true,
        "parse check with include-checks",
    );
}

#[test]
fn parse_check_list_checks() {
    let args = parse_ok(["prosesmasher", "check", "--list-checks"]);
    assert_check_command(
        &args,
        None,
        true,
        ConfigSource::None,
        None,
        None,
        "text",
        "failures",
        false,
        "parse check list-checks",
    );
}

#[test]
fn parse_check_with_text_mode() {
    let args = parse_ok([
        "prosesmasher",
        "check",
        "foo.md",
        "--preset",
        "general-en",
        "--text-mode",
        "summary",
    ]);
    assert_check_command(
        &args,
        Some("foo.md"),
        false,
        ConfigSource::Preset("general-en"),
        None,
        None,
        "text",
        "summary",
        false,
        "parse check with summary text mode",
    );
}

#[test]
fn parse_list_presets() {
    let args = parse_ok(["prosesmasher", "list-presets"]);
    assert_list_presets_command(&args, "parse list-presets");
}

#[test]
fn parse_dump_config_full() {
    let args = parse_ok(["prosesmasher", "dump-config", "--full-config"]);
    assert_dump_config_full_command(&args, "parse dump-config --full-config");
}

#[test]
fn parse_dump_config_preset() {
    let args = parse_ok(["prosesmasher", "dump-config", "--preset", "tweet-en"]);
    assert_dump_config_preset_command(&args, "tweet-en", "parse dump-config --preset");
}

#[test]
fn parse_dump_config_requires_source() {
    assert_parse_err(
        ["prosesmasher", "dump-config"],
        "dump-config should require a source",
    );
}

#[test]
fn parse_dump_config_conflicting_sources_fail() {
    assert_parse_err(
        [
            "prosesmasher",
            "dump-config",
            "--full-config",
            "--preset",
            "tweet-en",
        ],
        "dump-config sources should conflict",
    );
}
