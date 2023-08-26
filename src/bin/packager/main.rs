use conformator::*;
use std::{fs::File, io::Read};

use color_eyre::Result;
use nom::error;

fn main() -> Result<()> {
    color_eyre::install()?;
    let mut file = File::open("sample.txt").unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();
    let output = lexer::parser_prime::<error::VerboseError<&str>>(&data);

    let output: types::Concepts = output.unwrap().1.into();

    // println!("---\nAST:\n{:#?}", output);

    let levels = packages::construct_staged_list(output.packages)?;

    println!("---\npackage levels:\n{:#?}", levels);

    let functions = functions::function_registry_composer(output.functions)?;

    println!("---\nfunction execution");
    let output = functions
        .get("main")
        .map(|func| func(""))
        .unwrap_or(String::from("-1"));
    println!("[Exited with \"{}\"]", output);

    Ok(())
}
