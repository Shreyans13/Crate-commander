use std::path::PathBuf;
use::clap::Parser;


#[derive(Parser, Debug)]
#[clap()]
pub struct Options {
    pub args: Vec<String>,
    
    #[clap(short='c', long="config")]
    pub configs: Option<PathBuf>,
    
    #[clap(short='p', long="pwd")]
    pub pwd: Option<PathBuf>,
}
