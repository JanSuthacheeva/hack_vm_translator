use hack_vm_translator::translate;
use std::env;
use std::error::Error;
use std::fs;
use std::path::{PathBuf, Path};
use std::process;

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
    let mut output = String::from("");
    for input_file in &config.input_files {
        let input = fs::read_to_string(input_file)?;

        let name: &str = if input_file.is_relative() {
            input_file.strip_prefix("./")
                .expect("could not strip prefix")
                .to_str().
                ok_or("Could not transform input file to str")?
        } else {
            input_file.to_str().ok_or("Could not transform input file to str")?
        };
        output.push_str(&format!("// {name}\n"));
        let file_output = translate(&input, name)?;

        output.push_str(&file_output);
        output.push_str("\n");
    }


    fs::write(config.output_file, output)?;
    Ok(())
}

struct Config {
    input_files: Vec<PathBuf>,
    output_file: PathBuf,
}

impl Config {
    fn build(mut args: impl Iterator<Item = String>) -> Result<Config, Box<dyn Error>> {
        args.next();

        let input = match args.next() {
            Some(arg) => arg,
            None => return Err("Did not find an input".into()),
        };

        let path = Path::new(&input);

        if path.is_file() {
            if path.extension().unwrap() != "vm" {
                return Err("input must be a directory of .vm file".into());
            }

            return Ok(Config {
                input_files: vec![path.to_path_buf()],
                output_file: path.with_extension("asm").into(),
            })
        } 
        if path.is_dir() {
            let mut input_files: Vec<PathBuf> = vec![];
            handle_dir(path, &mut input_files);
            return Ok(Config {
                input_files,
                output_file: path.with_extension("asm").into(),
            });
        }

        return Err("test".into());
        
    }
}

fn handle_dir(path: &Path, input_files: &mut Vec<PathBuf>) {
    for entry in path.read_dir().expect("Failed to read directory") {
        if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_file() {
                    if path.extension().unwrap() == "vm" {
                        input_files.push(path.to_path_buf());
                    }
                }
                if path.is_dir() {
                    handle_dir(&path, input_files);
                }
        }
    }
}
