use std::fs::{self, File};
use std::process::Command;

use difference::Changeset;

pub fn assert_compiled_output_eq(source_path: &str, expected_path: &str) {
    let expected = std::fs::read_to_string(expected_path).unwrap();

    let dir = tempfile::tempdir().unwrap();
    let tmppath = dir.path().join("temp");
    let tmpfile = File::create(&tmppath).unwrap();

    Command::new("./target/debug/silica")
        .arg("-i")
        .arg(&source_path)
        .arg("-o")
        .arg(&tmppath)
        .output()
        .unwrap();

    let generated = fs::read_to_string(&tmppath).unwrap();

    drop(tmpfile);
    dir.close().unwrap();

    let changeset = Changeset::new(&generated, &expected, "");
    if changeset.distance > 0 {
        println!("{}", changeset);
        panic!("expected and generated sources differ");
    }
}

pub fn assert_compiler_passed(source_path: &str) {
    let dir = tempfile::tempdir().unwrap();
    let tmppath = dir.path().join("temp");
    let tmpfile = File::create(&tmppath).unwrap();

    let output = Command::new("./target/debug/silica")
        .arg("-i")
        .arg(&source_path)
        .arg("-o")
        .arg(&tmppath)
        .output()
        .unwrap();

    drop(tmpfile);
    dir.close().unwrap();

    if output.status.success() {
        panic!("compiler was expected to fail, but succeeded");
    }
}
