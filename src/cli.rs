use clap::Parser;

#[derive(Parser, Debug)]
pub struct ConfigArgs {
    /// Path to the projects config file
    #[clap(short, long, default_value = "configs")]
    pub config_folder: String,
}

/// CLI for generating jpegs
#[derive(Parser, Debug)]
pub enum Commands {
    /// Clean the output directory
    Clean,
    /// Generate an NFT collection
    Gen(ConfigArgs),
}

impl Default for Commands {
    fn default() -> Self {
        Self::new()
    }
}

impl Commands {
    pub fn new() -> Self {
        Commands::parse()
    }
}
