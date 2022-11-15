use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Serve {
        #[arg(short, long)]
        backend: Option<String>,
    },
}

pub fn cli() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Serve { backend: _ }) => {}
        None => {}
    }

}