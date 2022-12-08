use rust_sender::commands::CLI; 
use clap::Parser;

fn main() -> anyhow::Result<()> {
    // Parse the given arguments.
    let cli = CLI::parse();
    
    // Run the CLI.
    match cli.command.parse() {
        Ok(output) => println!("{output}\n"),
        Err(error) => println!("⚠️  {error}\n"),
    }
    Ok(())
}
