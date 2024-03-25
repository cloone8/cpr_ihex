use std::path::PathBuf;

use clap::{Args, Parser, ValueEnum};

#[derive(Parser, Debug)]
#[command(author, about, version)]
pub(crate) struct CLIArgs {
    /// The hex file to parse.
    #[arg()]
    pub file: Option<PathBuf>,

    #[command(flatten)]
    pub commands: CLICommands,

    /// The verbosity of the logger
    #[cfg(not(debug_assertions))]
    #[arg(value_enum, short, long, default_value_t = LogLevel::Warn)]
    pub verbosity: LogLevel,

    /// The verbosity of the logger
    #[cfg(debug_assertions)]
    #[arg(value_enum, short, long, default_value_t = LogLevel::Info)]
    pub verbosity: LogLevel,
}

#[derive(Args, Debug)]
#[group(id = "commands", multiple = false, requires = "file")]
pub(crate) struct CLICommands {
    /// If set, the program will output a prettified hexdump of the file instead of opening the GUI.
    #[arg(short = 'x', long)]
    pub hexdump: bool,

    /// If set, the program will output a binary dump of the file instead of opening the GUI.
    #[arg(short, long)]
    pub bindump: bool,
}

#[derive(Debug, Clone, ValueEnum)]
pub(crate) enum LogLevel {
    Error,
    Warn,
    Info,
    #[cfg(debug_assertions)]
    Debug,
    #[cfg(debug_assertions)]
    Trace,
}

impl From<LogLevel> for log::Level {
    fn from(log_level: LogLevel) -> Self {
        match log_level {
            LogLevel::Error => log::Level::Error,
            LogLevel::Warn => log::Level::Warn,
            LogLevel::Info => log::Level::Info,
            #[cfg(debug_assertions)]
            LogLevel::Debug => log::Level::Debug,
            #[cfg(debug_assertions)]
            LogLevel::Trace => log::Level::Trace,
        }
    }
}

impl From<LogLevel> for log::LevelFilter {
    fn from(log_level: LogLevel) -> Self {
        match log_level {
            LogLevel::Error => log::LevelFilter::Error,
            LogLevel::Warn => log::LevelFilter::Warn,
            LogLevel::Info => log::LevelFilter::Info,
            #[cfg(debug_assertions)]
            LogLevel::Debug => log::LevelFilter::Debug,
            #[cfg(debug_assertions)]
            LogLevel::Trace => log::LevelFilter::Trace,
        }
    }
}
