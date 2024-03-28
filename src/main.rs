mod cli;
// mod gui;
mod record;
mod utils;
mod gui;
use anyhow::{anyhow, Result};
use cli::{args::CLIArgs, commands::run_commands};
use record::file::IHexFile;
// use gui::Gui;


use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use clap::Parser;
use eframe::{egui::ViewportBuilder, NativeOptions};
use simplelog::{ColorChoice, ConfigBuilder, TermLogger, TerminalMode};

fn setup() -> Result<(CLIArgs, Option<File>)> {
    color_backtrace::install();

    let args = cli::args::CLIArgs::parse();

    let logconfig = ConfigBuilder::new()
        .set_time_format_rfc3339()
        .set_time_offset_to_local()
        .expect("Could not set time offset to local")
        .build();

    TermLogger::init(
        args.verbosity.clone().into(),
        logconfig,
        TerminalMode::Stderr,
        ColorChoice::Auto,
    )
    .expect("Could not initialize logger");

    log::info!("Logger initialized");

    let file = match &args.file {
        Some(file) => Some(
            File::open(file)
                .map_err(|e| anyhow!("I/O Error while opening provided file: {}", e))?,
        ),
        None => None,
    };

    Ok((args, file))
}

fn main() -> Result<()> {
    let (args, provided_file) = setup().map_err(|e| anyhow!("Setup failed: {}", e))?;
    let parsed_file = match provided_file {
        Some(file) => Some(IHexFile::read(BufReader::new(file).lines())?),
        None => None,
    };

    run_commands(&args, parsed_file.as_ref())?;

    // let native_options = NativeOptions {
    //     viewport: ViewportBuilder::default()
    //         .with_title("CPR IHEX - Intel HEX Parser and Manipulator"),
    //     ..Default::default()
    // };

    // eframe::run_native(
    //     "io.wutru.cpr_ihex",
    //     native_options,
    //     Box::new(move |cc| Box::new(Gui::new(cc, parsed_file))),
    // )
    // .expect("Could not run GUI");

    Ok(())
}
