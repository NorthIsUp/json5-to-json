use assert_cmd::Command;
use predicates::str::contains;
use std::fs;
use tempfile::TempDir;

fn cmd() -> Command {
    Command::cargo_bin("json5-to-json").unwrap()
}

// --- stdin ---

#[test]
fn stdin_basic() {
    cmd()
        .write_stdin("{ a: 1 }")
        .assert()
        .success()
        .stdout("{\n  \"a\": 1\n}\n");
}

#[test]
fn stdin_compact() {
    cmd()
        .args(["-n", "0"])
        .write_stdin("{ a: 1 }")
        .assert()
        .success()
        .stdout("{\"a\":1}\n");
}

#[test]
fn stdin_explicit_dash() {
    cmd()
        .arg("-")
        .write_stdin("{ a: 1 }")
        .assert()
        .success()
        .stdout("{\n  \"a\": 1\n}\n");
}

// --- file input ---

#[test]
fn file_input() {
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("input.json5");
    fs::write(&path, "{ hello: \"world\" }").unwrap();

    cmd()
        .arg(&path)
        .assert()
        .success()
        .stdout("{\n  \"hello\": \"world\"\n}\n");
}

#[test]
fn multiple_files() {
    let dir = TempDir::new().unwrap();
    let one = dir.path().join("one.json5");
    let two = dir.path().join("two.json5");
    fs::write(&one, "{ a: 1 }").unwrap();
    fs::write(&two, "{ b: 2 }").unwrap();

    cmd()
        .args(["-n", "0"])
        .arg(&one)
        .arg(&two)
        .assert()
        .success()
        .stdout("{\"a\":1}\n{\"b\":2}\n");
}

// --- sort ---

#[test]
fn sort_keys() {
    cmd()
        .args(["-s", "-n", "0"])
        .write_stdin("{ z: 1, a: 2 }")
        .assert()
        .success()
        .stdout("{\"a\":2,\"z\":1}\n");
}

#[test]
fn no_sort_does_not_error() {
    cmd()
        .args(["--no-sort", "-n", "0"])
        .write_stdin("{ z: 1, a: 2 }")
        .assert()
        .success();
}

// --- indent ---

#[test]
fn indent_4() {
    cmd()
        .args(["-n", "4"])
        .write_stdin("{ a: 1 }")
        .assert()
        .success()
        .stdout("{\n    \"a\": 1\n}\n");
}

#[test]
fn indent_0() {
    cmd()
        .args(["--indent", "0"])
        .write_stdin("{ a: 1 }")
        .assert()
        .success()
        .stdout("{\"a\":1}\n");
}

#[test]
fn indent_auto_explicit() {
    cmd()
        .args(["--indent", "auto"])
        .write_stdin("{ a: 1 }")
        .assert()
        .success()
        .stdout("{\n  \"a\": 1\n}\n");
}

#[test]
fn indent_auto_detects_4() {
    cmd()
        .args(["--indent", "auto"])
        .write_stdin("{\n    a: 1\n}")
        .assert()
        .success()
        .stdout("{\n    \"a\": 1\n}\n");
}

// --- inplace ---

#[test]
fn inplace_writes_file() {
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("inplace.json5");
    fs::write(&path, "{ x: 42 }").unwrap();

    cmd()
        .args(["-i", "-n", "0"])
        .arg(&path)
        .assert()
        .success();

    assert_eq!(fs::read_to_string(&path).unwrap(), "{\"x\":42}\n");
}

#[test]
fn inplace_multiple_files() {
    let dir = TempDir::new().unwrap();
    let one = dir.path().join("ip1.json5");
    let two = dir.path().join("ip2.json5");
    fs::write(&one, "{ a: 1 }").unwrap();
    fs::write(&two, "{ b: 2 }").unwrap();

    cmd()
        .args(["-i", "-n", "0"])
        .arg(&one)
        .arg(&two)
        .assert()
        .success();

    assert_eq!(fs::read_to_string(&one).unwrap(), "{\"a\":1}\n");
    assert_eq!(fs::read_to_string(&two).unwrap(), "{\"b\":2}\n");
}

#[test]
fn inplace_rejects_stdin() {
    cmd()
        .args(["-i", "-"])
        .assert()
        .failure();
}

// --- check ---

#[test]
fn check_dirty_file_exits_1() {
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("dirty.json5");
    fs::write(&path, "{ needs: \"change\" }").unwrap();

    cmd()
        .arg("--check")
        .arg(&path)
        .assert()
        .failure()
        .stderr(contains(path.to_str().unwrap()));
}

#[test]
fn check_clean_file_exits_0() {
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("clean.json");
    fs::write(&path, "{\n  \"clean\": true\n}\n").unwrap();

    cmd()
        .arg("--check")
        .arg(&path)
        .assert()
        .success();
}

#[test]
fn check_mixed_files_exits_1() {
    let dir = TempDir::new().unwrap();
    let clean = dir.path().join("c1.json");
    let dirty = dir.path().join("c2.json5");
    fs::write(&clean, "{\n  \"ok\": 1\n}\n").unwrap();
    fs::write(&dirty, "{ dirty: 1 }").unwrap();

    cmd()
        .arg("--check")
        .arg(&clean)
        .arg(&dirty)
        .assert()
        .failure()
        .stderr(contains(dirty.to_str().unwrap()));
}

#[test]
fn check_does_not_modify_file() {
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("nomod.json5");
    fs::write(&path, "{ a: 1 }").unwrap();

    cmd()
        .arg("--check")
        .arg(&path)
        .assert()
        .failure();

    assert_eq!(fs::read_to_string(&path).unwrap(), "{ a: 1 }");
}

#[test]
fn check_rejects_stdin() {
    cmd()
        .args(["--check", "-"])
        .assert()
        .failure();
}

// --- check + inplace ---

#[test]
fn check_inplace_does_not_modify() {
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("both.json5");
    fs::write(&path, "{ a: 1 }").unwrap();

    cmd()
        .args(["--check", "-i"])
        .arg(&path)
        .assert()
        .failure();

    assert_eq!(fs::read_to_string(&path).unwrap(), "{ a: 1 }");
}

// --- errors ---

#[test]
fn nonexistent_file_exits_1() {
    cmd()
        .arg("/tmp/nonexistent_json5_file")
        .assert()
        .failure();
}

#[test]
fn malformed_json5_exits_1() {
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("bad.json5");
    fs::write(&path, "{ broken").unwrap();

    cmd()
        .arg(&path)
        .assert()
        .failure();
}
