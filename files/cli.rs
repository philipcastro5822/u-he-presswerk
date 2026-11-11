//! Minimal command-line interface for the U He Presswerk helper binary.

use std::env;

pub enum Command {
    Version,
    Help,
    Install,
    Repair,
    Uninstall,
    Unknown(String),
}

pub fn parse() -> Command {
    let args: Vec<String> = env::args().skip(1).collect();
    match args.first().map(|s| s.as_str()).unwrap_or("") {
        "--version" | "-v" => Command::Version,
        "--help"    | "-h" => Command::Help,
        "install"          => Command::Install,
        "repair"           => Command::Repair,
        "uninstall"        => Command::Uninstall,
        other              => Command::Unknown(other.to_string()),
    }
}

pub fn help_text() -> &'static str {
    "U He Presswerk v4.6.2 - Windows helper\n\
     \n\
     USAGE:\n\
         u-he-presswerk-helper <COMMAND>\n\
     \n\
     COMMANDS:\n\
         install    Install or update U He Presswerk\n\
         repair     Repair a broken installation\n\
         uninstall  Remove U He Presswerk and its config\n\
         --version  Print version and exit\n\
         --help     Show this help\n"
}
