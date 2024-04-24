use crate::{
    args::*, get::get, list::list, new, randompass::random, remove::remove
};

pub fn match_args(args: Args) {
    match args.command {
        Commands::New { application } => {
            new::new(application);
        }
        Commands::Random => {
            println!("{}", random());
        }
        Commands::Get { application } => {
            println!("password for {}: {}",
                application, 
                get(application.clone()));
        }
        Commands::List => {
            list();
        }
        Commands::Remove { application } => {
            remove(application);
        }
    }
}