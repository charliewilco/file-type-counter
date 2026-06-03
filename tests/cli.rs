use serde_json::Value;
use std::fs;
use std::path::Path;
use std::process::Command;
use tempfile::TempDir;

fn run(args: &[&str]) -> String {
    let output = Command::new(env!("CARGO_BIN_EXE_extension-count"))
        .args(args)
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("run extension-count");

    assert!(
        output.status.success(),
        "command failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    String::from_utf8(output.stdout).expect("stdout utf8")
}

fn write_file(path: &Path, contents: &str) {
    fs::write(path, contents).expect("write file");
}

#[test]
fn renders_plain_table_snapshot() {
    let output = run(&[
        "fixture",
        "--ci",
        "--no-labels",
        "--sort",
        "ext",
        "--limit",
        "0",
    ]);

    assert_eq!(
        output,
        [
            "Results for: fixture",
            "Total files: 1",
            "Extension  Count  Files           ",
            "---------  -----  ----------------",
            ".ts            1  fixture/index.ts",
            "",
            "",
        ]
        .join("\n")
    );
}

#[test]
fn renders_bordered_table_snapshot() {
    let output = run(&[
        "fixture",
        "--ci",
        "--no-labels",
        "--sort",
        "ext",
        "--limit",
        "0",
        "--bordered",
    ]);

    assert_eq!(
        output,
        [
            "Results for: fixture",
            "Total files: 1",
            "+-----------+-------+------------------+",
            "| Extension | Count | Files            |",
            "+-----------+-------+------------------+",
            "| .ts       |     1 | fixture/index.ts |",
            "+-----------+-------+------------------+",
            "",
            "",
        ]
        .join("\n")
    );
}

#[test]
fn renders_summary_table_snapshot() {
    let output = run(&[
        "fixture",
        "--ci",
        "--no-labels",
        "--sort",
        "ext",
        "--summary",
    ]);

    assert_eq!(
        output,
        [
            "Results for: fixture",
            "Total files: 1",
            "Extension  Count",
            "---------  -----",
            ".ts            1",
            "",
            "",
        ]
        .join("\n")
    );
}

#[test]
fn renders_summary_json_without_files() {
    let output = run(&[
        "fixture",
        "--ci",
        "--no-labels",
        "--sort",
        "ext",
        "--summary",
        "--json",
    ]);
    let parsed: Value = serde_json::from_str(&output).expect("json");

    assert_eq!(parsed[0]["rows"][0]["extension"], ".ts");
    assert_eq!(parsed[0]["rows"][0]["count"], 1);
    assert!(parsed[0]["rows"][0].get("files").is_none());
}

#[test]
fn json_output_respects_requested_sorting() {
    let temp = TempDir::new().expect("temp dir");
    write_file(&temp.path().join("one.rs"), "");
    write_file(&temp.path().join("two.ts"), "");
    write_file(&temp.path().join("three.ts"), "");

    let output = run(&[
        temp.path().to_str().expect("temp path"),
        "--json",
        "--summary",
        "--no-labels",
        "--sort",
        "count",
    ]);
    let parsed: Value = serde_json::from_str(&output).expect("json");

    assert_eq!(parsed[0]["rows"][0]["extension"], ".ts");
    assert_eq!(parsed[0]["rows"][0]["count"], 2);
    assert_eq!(parsed[0]["rows"][1]["extension"], ".rs");
    assert_eq!(parsed[0]["rows"][1]["count"], 1);
}

#[test]
fn supports_custom_label_files() {
    let temp = TempDir::new().expect("temp dir");
    let labels = temp.path().join("labels.json");
    write_file(&labels, r#"{"ts":"Fixture TypeScript"}"#);

    let output = run(&[
        "fixture",
        "--ci",
        "--labels",
        labels.to_str().expect("labels path"),
        "--summary",
    ]);

    assert!(output.contains("Fixture TypeScript"));
    assert!(!output.contains("fixture/index.ts"));
}

#[test]
fn displays_files_without_extensions() {
    let temp = TempDir::new().expect("temp dir");
    write_file(&temp.path().join("README"), "readme");

    let output = run(&[
        temp.path().to_str().expect("temp path"),
        "--ci",
        "--no-labels",
        "--summary",
    ]);

    assert!(output.contains("(none)"));
}

#[test]
fn respects_ignore_filters_by_default() {
    let temp = TempDir::new().expect("temp dir");
    fs::create_dir_all(temp.path().join("node_modules/pkg")).expect("node modules");
    fs::create_dir_all(temp.path().join("target/debug")).expect("target");
    fs::create_dir_all(temp.path().join(".hidden")).expect("hidden");
    write_file(&temp.path().join(".gitignore"), "*.tmp\n");
    write_file(&temp.path().join("kept.rs"), "");
    write_file(&temp.path().join("ignored.tmp"), "");
    write_file(&temp.path().join("node_modules/pkg/index.js"), "");
    write_file(&temp.path().join("target/debug/output.o"), "");
    write_file(&temp.path().join(".hidden/secret.md"), "");

    let output = run(&[
        temp.path().to_str().expect("temp path"),
        "--json",
        "--summary",
        "--no-labels",
    ]);
    let parsed: Value = serde_json::from_str(&output).expect("json");

    assert_eq!(parsed[0]["total_files"], 1);
    assert_eq!(parsed[0]["rows"][0]["extension"], ".rs");
}

#[test]
fn can_disable_ignore_filters() {
    let temp = TempDir::new().expect("temp dir");
    fs::create_dir_all(temp.path().join("node_modules/pkg")).expect("node modules");
    fs::create_dir_all(temp.path().join("target/debug")).expect("target");
    fs::create_dir_all(temp.path().join(".hidden")).expect("hidden");
    write_file(&temp.path().join(".gitignore"), "*.tmp\n");
    write_file(&temp.path().join("kept.rs"), "");
    write_file(&temp.path().join("ignored.tmp"), "");
    write_file(&temp.path().join("node_modules/pkg/index.js"), "");
    write_file(&temp.path().join("target/debug/output.o"), "");
    write_file(&temp.path().join(".hidden/secret.md"), "");

    let output = run(&[
        temp.path().to_str().expect("temp path"),
        "--json",
        "--summary",
        "--no-labels",
        "--no-ignore",
    ]);
    let parsed: Value = serde_json::from_str(&output).expect("json");

    assert_eq!(parsed[0]["total_files"], 6);
}
