use conformator::*;
use std::{fs::File, io::Read};

use color_eyre::Result;
use nom::error;
use std::env;

fn main() -> Result<()> {
    color_eyre::install()?;
    let filename = env::args().nth(1).expect("No file provided");
    let mut file = File::open(filename).expect("Something went wrong while opening the file");
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();
    let output = lexer::parser_prime::<error::VerboseError<&str>>(&data);

    let output: types::Concepts = output.unwrap().1.into();

    let functions = functions::function_registry_composer(output.functions)?;

    let output = functions
        .get("main")
        .map(|func| func(""))
        .unwrap_or(String::from("-1"));
    println!("[Exited with \"{}\"]", output);

    Ok(())
}
