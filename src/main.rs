use std::fs::read_to_string;

use clap::Parser;
use exofork::frontend::parser::parse;

#[derive(Parser)]
struct Args {
    #[arg()]
    input_file: String,
    #[arg(short, long, default_value_t = String::from("out.o"))]
    output_file: String
}

fn main() {
    let args = Args::parse();

    let src = read_to_string(args.input_file).unwrap();

    let ast = parse(&src);
}