mod utils;
mod projector;

use::clap::Parser;
use crate::utils::config::Config;
use crate::utils::utils::Options;
use crate::projector::projector::Projector;

use anyhow::Result;


fn main() -> Result<()> {
    let config:Config = Options::parse().try_into()?; 
    let mut proj = Projector::from_config(config.config, config.pwd);

    match config.operation {
        utils::config::Operation::Print(None) => {
            let value = proj.get_value_all();
            let value = serde_json::to_string(&value);
            println!("{:?}", value);
        },
        utils::config::Operation::Print(Some(v)) => {
            proj.get_value(&v).map(|x| {
                println!("{:?}", x);
            });
        },
        utils::config::Operation::Add(k, v) => {
            proj.set_value(k,v);
            proj.save()?;
        },
        utils::config::Operation::Remove(k) => {
            proj.delete_value(&k);
            proj.save()?;
        },
    }
    return Ok(());
}
