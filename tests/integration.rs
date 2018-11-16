use std::fs::File;
use std::io::Read;
use std::process::Command;

#[test]
fn and_gate() {

    Command::new("./target/debug/silicac")
        .arg("-i")
        .arg("./tests/resources/and_gate/and_gate.si")
        .arg("-o")
        .arg("./tests/resources/and_gate/generated_and_gate.v")
        .output()
        .expect("Failed to execute process.");

    let mut expected_file = File::open("./tests/resources/and_gate/and_gate.v")
        .expect("Input file not found.");
    let mut expected_verilog = String::new();
    expected_file.read_to_string(&mut expected_verilog)
        .expect("Unable to read the file.");

    let mut generated_file = File::open("./tests/resources/and_gate/generated_and_gate.v")
        .expect("Output file not found.");
    let mut generated_verilog = String::new();
    generated_file.read_to_string(&mut generated_verilog)
        .expect("Unable to read the file.");

    assert_eq!(expected_verilog, generated_verilog);
}