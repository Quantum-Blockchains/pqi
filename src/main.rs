use pqi::{cli, config};
use std::error;

fn main() -> Result<(), Box<dyn error::Error>> {
    let args = cli::Args::fron_args();

    let config = config::Config::build(args.config_file)?;
    println!("{:?}", config);
    Ok(())
}
