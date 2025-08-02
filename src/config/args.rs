use clap::Parser;

/// A terminal-based portfolio viewer that fetches data from GitHub
#[derive(Parser, Debug, Clone)]
#[clap(author, version, about)]
pub struct Args {
    /// Path to the config file (local or GitHub URL)
    #[clap(short, long, default_value = "https://github.com/Pokeylooted/Pokeylooted.github.io/blob/main/_config.yml")]
    pub config_path: String,
}