use args::Args;
use clap::Parser;
use crate::checkifran::checkifran;
use color_eyre::eyre::Result;
use match_args::match_args;

mod append_to_path;
mod args;
mod checkifran;
mod get;
mod list;
mod match_args;
mod new;
mod randompass;
mod remove;
mod rpass_dir;

fn main() -> Result<()> {
    color_eyre::install()?;
    
    checkifran();
    let args = Args::parse();
    match_args(args);

    Ok(())
}
