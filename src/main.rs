use std::error::Error;
use std::env;
use std::fs;
use std::process;
use std::path::{PathBuf};
use hack_vm_translator::translate;

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {

    let input = fs::read_to_string(config.input_file)?;

    let output = translate(&input)?;

    fs::write(config.output_file, output)?;

    Ok(())
}

struct Config {
    input_file: PathBuf,
    output_file: PathBuf,
}

impl Config {

    fn build(mut args: impl Iterator<Item = String>) -> Result<Config, Box<dyn Error>> {
        args.next();

        let input_file = match args.next() {
            Some(arg) => arg,
            None => return Err("Did not find an input file".into()),
        };

        let stem = input_file
            .strip_suffix(".vm")
            .ok_or("input must be a .vm file")?;

        let output_file = format!("{stem}.asm");


        Ok(Config {
            input_file: input_file.into(),
            output_file: output_file.into(),
        })
    }
}
