use std::fs::File;
use std::io::Read;
use std::io::Write;

fn main() {
    let matches = clap::App::new("silicac")
        .arg(
            clap::Arg::with_name("input")
                .short("i")
                .long("input")
                .takes_value(true)
                .value_name("FILE")
                .required(true)
                .help("The input file to compile"),
        )
        .arg(
            clap::Arg::with_name("output")
                .short("o")
                .long("output")
                .takes_value(true)
                .value_name("FILE")
                .required(true)
                .help("The output file to generate"),
        )
        .get_matches();

    let input_filename = matches
        .value_of("input")
        .expect("Failed to get input file path");
    let mut input_file = File::open(input_filename).expect("Input file not found");
    let mut input = String::new();
    input_file
        .read_to_string(&mut input)
        .expect("Failed to read file");

    let ast = syntax::parse_source(&input).expect("Failed to parse the source");
    let verilog = verilog::Verilog::new(&ast).build();

    let output_filename = matches
        .value_of("output")
        .expect("Failed to get output file path");
    let mut output_file = File::create(output_filename).expect("Unable to open output file");
    output_file
        .write_all(verilog.as_bytes())
        .expect("Unable to write to file");
}
