use clap::{Parser, Subcommand};

/// A password manager for all of your applications
#[derive(Debug, Parser)]
#[command(name = "rpass")]
#[command(about = "A password manager for all of your applications", long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Makes a new password and saves it to a file with the specified app
    #[command(arg_required_else_help = true)]
    New { application: String },
    /// Makes a random password and prints it out, it will not be saved
    Random,
    /// Fetches a password from the given application and prints it
    #[command(arg_required_else_help = true)]
    Get { application: String },
    /// Lists applications that have an assigned password
    List,
    /// Removes a password
    #[command(arg_required_else_help = true)]
    Remove{ application: String },
}