extern crate clap;

use std::fs::File;
use std::io::Read;

fn main() {

    let matches = clap::App::new("silicac")
        .arg(clap::Arg::with_name("input")
            .short("i")
            .long("input")
            .takes_value(true)
            .value_name("FILE")
            .required(true)
            .help("The input file to compile"))
        .get_matches();

    let input_filename = matches.value_of("input").unwrap();
    let mut input_file = File::open(input_filename).expect("Input file not found");
    let mut input = String::new();
    input_file.read_to_string(&mut input).expect("Failed to read file");

    println!("{}", input);

}
