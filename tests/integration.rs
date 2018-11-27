use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::process::Command;

#[test]
fn expressions() {

    let mut expected = String::new();
    let mut generated = String::new();

    let file = File::open("./tests/resources/expressions/expressions.v")
        .expect("Input file not found.");
    let file = BufReader::new(&file);
    for line in file.lines() {
        expected.push_str(line.unwrap().trim());
    }

    Command::new("./target/debug/silica")
        .arg("-i")
        .arg("./tests/resources/expressions/expressions.si")
        .arg("-o")
        .arg("./tests/resources/expressions/generated_expressions.v")
        .output()
        .expect("Failed to execute process.");

    let file = File::open("./tests/resources/expressions/generated_expressions.v")
        .expect("Generated file not found.");
    let file = BufReader::new(&file);
    for line in file.lines() {
        generated.push_str(line.unwrap().trim());
    }

    std::fs::remove_file("./tests/resources/expressions/generated_expressions.v")
        .expect("Failed to remove generated file.");

    assert_eq!(expected, generated);
}