mod ast;
mod error;
mod whetstone;

use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use clap::Parser;
use error::Error;

#[derive(Debug, Parser)]
struct Args {
    file: Option<PathBuf>,
}

fn main() -> Result<(), Error> {
    let args = Args::parse();

    match args.file {
        Some(path) => {
            let file = fs::read_to_string(path)?;
            let mut state = HashMap::new();
            whetstone::ScriptParser::new().parse(&mut state, &file).unwrap();
        },
        None => todo!(),
    }

    Ok(())
}
