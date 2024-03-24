use crate::ihex_file::IHexFile;
use pretty_hex::pretty_hex;
use std::{
    error::Error,
    fmt::{self, Display, Formatter},
    io::{stdout, BufWriter, Write},
};

use super::args::CLIArgs;

fn run_bindump(file: &IHexFile) -> Result<(), std::io::Error> {
    let mut writer = BufWriter::new(stdout());

    writer.write_all(&file.data_bytes())?;

    Ok(())
}

fn run_hexdump(file: &IHexFile) {
    println!("{}", pretty_hex(&file.data_bytes()));
}

#[derive(Debug)]
pub enum RunCommandErr {
    FileNotProvided,
    IoError(std::io::Error),
}

impl Display for RunCommandErr {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            RunCommandErr::FileNotProvided => write!(f, "No file was provided to run commands on"),
            RunCommandErr::IoError(e) => write!(f, "An I/O error occurred: {}", e),
        }
    }
}

impl Error for RunCommandErr {}

impl From<std::io::Error> for RunCommandErr {
    fn from(e: std::io::Error) -> Self {
        RunCommandErr::IoError(e)
    }
}

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
        run_bindump(file.unwrap())?;
        return Ok(true);
    }

    Ok(false)
}
