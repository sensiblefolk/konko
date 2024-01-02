use std::process::exit;
use error::exit_with_retcode;

#[macro_use] extern crate scan_fmt;

mod cli;
mod error;
mod config;
mod container;
mod ipc;
mod child;

fn main() {
    match cli::parse_args() {
        Ok(args) => {
            log::info!("{:?}", args);
            exit_with_retcode(container::start(args));
        }, Err(e) => {
            log::error!("Error while parsing arguments:\n\t{}", e);
            exit(e.get_retcode());
        }
    }
}
