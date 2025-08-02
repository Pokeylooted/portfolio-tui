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
    
    // Initialize the application
    let mut app = ui::app::App::new(args)?;
    
    // Run the application
    app.run().await?;
    
    Ok(())
}
