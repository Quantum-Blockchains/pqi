pub mod cli {
    use clap::Parser;
    use std::path::PathBuf;
    /// todo

    #[derive(Parser, Debug)]
    #[command(version, about, long_about = None)]
    pub struct Args {
        /// Path to config file
        #[arg(short = 'c', long = "config", value_name = "CONFIG_FILE")]
        pub config_file: PathBuf,
    }

    impl Args {
        pub fn fron_args() -> Args {
            Args::parse()
        }
    }
}

pub mod config {
    use serde::{Deserialize, Serialize};
    use std::{error, fs, path::PathBuf};

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Config {
        sae_id: String,
        kme_address: String,
        targets_sae_id: Vec<String>,
    }

    impl Config {
        pub fn build(config_path: PathBuf) -> Result<Config, Box<dyn error::Error>> {
            let data = fs::read(config_path)?;
            let text = String::from_utf8(data)?;
            let config: Config = toml::from_str(&text)?;
            Ok(config)
        }
    }
}
