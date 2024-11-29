mod saver;

use std::{env, fs};
use std::process::Command;
use std::str::FromStr;

enum ArgCategory {
    Help,
    Version,
    Verbose,
    Save,
    Load,
    Delete,
}

impl FromStr for ArgCategory {
    type Err = ();

    fn from_str(input: &str) -> Result<ArgCategory, Self::Err> {
        match input {
            "-h" | "--help" => Ok(ArgCategory::Help),
            "-v" | "--version" => Ok(ArgCategory::Version),
            "-V" | "--verbose" => Ok(ArgCategory::Verbose),
            "-S" | "--save" => Ok(ArgCategory::Save),
            "-L" | "--load" => Ok(ArgCategory::Load),
            "-D" | "--delete" => Ok(ArgCategory::Delete),
            _ => Err(()),
        }
    }
}

fn help() {
    println!("
  ksesh - Kitty session saver/loader.
  USAGE: ksesh [options] <session path>
    OPTIONS:
    -h, --help      : Displays the help menu.
    -v, --version   : Displays the version of the software.
    -V, --verbose   : Enable verbosity.
    -S, --save      : Saves the current kitty session to the specified session path.
    -L, --load      : Loads the specified session to a kitty session.
    -D, --delete    : Deletes the specified session file.\
    \n");
}

fn version() {
    const VERSION: &str = env!("CARGO_PKG_VERSION");

    println!("ksesh - Version {VERSION}");
}

fn load(session_path: &str) {
    match Command::new("kitty")
        .arg("--session")
        .arg(format!("~/.config/kitty/sessions/{session_path}.kitty"))
        .spawn() {
        Ok(_) => println!("Successfully loaded the session."),
        Err(error) => println!("An error occurred while loading! - {error}"),
    }
}

fn delete(session_path: &str) {
    let _ = fs::remove_file(
        format!("{}/.config/kitty/sessions/{}.kitty", env::var("HOME").unwrap(), session_path)
    );
}

fn main() {
    let args: Vec<String> = env::args().collect();

    for arg in &args[1..] {
        let category: ArgCategory = match ArgCategory::from_str(arg) {
            Ok(cat) => cat,
            Err(_) => {
                println!("INVALID ARGUMENT: {arg}");
                help();
                break;
            },
        };

        match category {
            ArgCategory::Help => {
                help();
                break;
            },
            ArgCategory::Version => {
                version();
                break;
            },
            ArgCategory::Verbose => {},
            ArgCategory::Save => {
                saver::save_session(&args[args.len() - 1]);
                break;
            },
            ArgCategory::Load => {
                load(&args[args.len() - 1]);
                break;
            },
            ArgCategory::Delete => {
                delete(&args[args.len() - 1]);
                break;
            }
        };
    }
}
