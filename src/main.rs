#![feature(naked_functions)]

mod config;
mod emu;

use emu::Executor;

use clap::{Parser, Subcommand};
use env_logger::{Builder, Env};

include!(concat!(env!("OUT_DIR"), "/instructions.rs"));


#[derive(Subcommand)]
pub enum Command {
    Exec {
        path: String,
    },
    Trace {
        path: String,
    },
}

#[derive(Parser)]
pub struct Args {
    #[command(subcommand)]
    command: Command,

    #[arg(long, short)]
    silent: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Builder::from_env(Env::default().default_filter_or("info")).init();

    let args = Args::parse();

    match args.command {
        Command::Exec { path } => {
            let mut executor = Executor::new(&path)?;

            executor.exec();
        },
        Command::Trace { path } => {
        },
    }

    Ok(())
}


