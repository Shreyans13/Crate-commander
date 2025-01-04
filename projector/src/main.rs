mod utils;
mod projector;

use::clap::Parser;
use crate::utils::config::Config;
use crate::utils::utils::Options;
use anyhow::Result;


fn main() -> Result<()> {
    let opts:Config = Options::parse().try_into()?; 
    println!("{:?}", opts);

    return Ok(());
}
