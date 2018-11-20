use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::process::Command;

#[test]
fn and_gate() {

    let mut expected = String::new();
    let mut generated = String::new();

    let file = File::open("./tests/resources/and_gate/and_gate.v")
        .expect("Input file not found.");
    let file = BufReader::new(&file);
    for line in file.lines() {
        expected.push_str(line.unwrap().trim());
    }

    Command::new("./target/debug/silica")
        .arg("-i")
        .arg("./tests/resources/and_gate/and_gate.si")
        .arg("-o")
        .arg("./tests/resources/and_gate/generated_and_gate.v")
        .output()
        .expect("Failed to execute process.");

    let file = File::open("./tests/resources/and_gate/generated_and_gate.v")
        .expect("Generated file not found.");
    let file = BufReader::new(&file);
    for line in file.lines() {
        generated.push_str(line.unwrap().trim());
    }

    std::fs::remove_file("./tests/resources/and_gate/generated_and_gate.v")
        .expect("Failed to remove generated file.");

    assert_eq!(expected, generated);
}