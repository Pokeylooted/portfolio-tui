mod config;
mod data;
mod processor;
mod ui;

use anyhow::Result;
use clap::Parser;
use config::args::Args;

#[tokio::main]
async fn main() -> Result<()> {
    // Parse command line arguments
    let args = Args::parse();
    
    // Check if the config path is a local file that exists
    if !args.config_path.starts_with("http") && std::path::Path::new(&args.config_path).exists() {
        println!("Using local config file: {}", args.config_path);
    } else if args.config_path.starts_with("http") {
        println!("Using remote config file: {}", args.config_path);
    } else {
        println!("Warning: Config file not found at: {}", args.config_path);
        println!("Will attempt to use default GitHub config.");
    }
    
    // Initialize the application
    let mut app = ui::app::App::new(args)?;
    
    // Run the application
    app.run().await?;
    
    Ok(())
}
