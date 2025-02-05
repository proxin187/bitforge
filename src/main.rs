#![feature(naked_functions)]
#![feature(iter_next_chunk)]

mod instruction;
mod config;
mod emu;

use emu::Executor;

use clap::{Parser, Subcommand};
use env_logger::{Builder, Env};
use log::info;

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
    let args = Args::parse();

    if !args.silent {
        Builder::from_env(Env::default().default_filter_or("info")).init();
    }

    match args.command {
        Command::Exec { path } => {
            let mut executor = Executor::new(&path)?;

            executor.run()?;
        },
        Command::Trace { path } => {
        },
    }

    Ok(())
}


