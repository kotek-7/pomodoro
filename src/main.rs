mod helper;
mod timer;

use clap::{Parser, Subcommand};
use timer::start;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Start,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Start => {
            println!("pomodoro start!");
            start();
            println!("pomodoro finish!");
        }
    }
}
