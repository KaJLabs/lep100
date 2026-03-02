use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name="lep100", about="LEP100 CLI (starter)")]
struct Cli {
    #[command(subcommand)]
    cmd: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Print version info
    Version,
    /// Placeholder for running conformance
    Conformance { #[arg(default_value="tests/lep100-tests")] path: String },
}

fn main() {
    let cli = Cli::parse();
    match cli.cmd {
        Command::Version => println!("lep100 (draft)"),
        Command::Conformance { path } => println!("Run Lithic conformance suite at: {path} (wire into lithc)"),
    }
}
