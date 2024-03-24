use std::{error::Error, fmt::{self, Display, Formatter}};

use crate::ihex_file::IHexFile;

use super::args::CLIArgs;

fn run_bindump(file: &IHexFile) {
    dbg!("bindump");
}

fn run_hexdump(file: &IHexFile) {
    dbg!("hexdump");
}

#[derive(Debug)]
pub enum RunCommandErr {
    FileNotProvided,
}

impl Display for RunCommandErr {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            RunCommandErr::FileNotProvided => write!(f, "No file was provided to run commands on"),
        }
    }
}

impl Error for RunCommandErr {}

fn check_file(file: Option<&IHexFile>) -> Result<(), RunCommandErr> {
    match file {
        Some(_) => Ok(()),
        None => Err(RunCommandErr::FileNotProvided),
    }
}

/// If any commands were specified in the args, run them.
/// Returns true if a command was run, false otherwise.
pub fn run_commands(args: &CLIArgs, file: Option<&IHexFile>) -> Result<bool, RunCommandErr> {
    //TODO: Convert all the verification to Clap arg groups
    //TODO: Make mutually exclusive
    if args.hexdump {
        check_file(file)?;
        run_hexdump(file.unwrap());
        return Ok(true);
    }

    if args.bindump {
        check_file(file)?;
        run_bindump(file.unwrap());
        return Ok(true);
    }

    Ok(false)
}
