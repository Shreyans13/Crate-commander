use anyhow::{ Result, anyhow, Context};
use crate::Options;
use std::path::PathBuf;

#[derive(Debug, PartialEq)]
pub enum Operation {
    Print(Option<String>),
    Add(String, String),
    Remove(String),
}

impl TryFrom<Vec<String>> for Operation {
    type Error = anyhow::Error;

    fn try_from(mut value: Vec<String>) -> Result<Self> {
        // let value = mut value;
        if value.len() == 0 {
            return Ok(Operation::Print(None));
        }

        let term = value.get(0).unwrap();
        if term == "add" {
            if value.len() != 3 {
                let err = anyhow!("Operation add expects 2 arguments but got {}", value.len() - 1);
                return Err(err);
            }
            let mut drain = value.drain(1..=2);
            return Ok(Operation::Add(
                    drain.next().expect("should exist"),
                    drain.next().expect("should exist"),
            ))
        }

        if term == "remove" {
            if value.len() != 2 {
                let err = anyhow!("Operation remove expects 1 arguments but got {}", value.len() - 1);
                return Err(err);
            }
            let arg = value.pop().expect("to exist");
            return Ok(Operation::Remove(arg))
        }

        if value.len() > 1 {
                let err = anyhow!("Operation print expects 0 or 1 arguments but got {}", value.len());
                return Err(err);
        }

        let arg = value.pop().expect("to exist");
        return Ok(Operation::Print(Some(arg)))
    }
        
}

#[derive(Debug)]
pub struct Config {
    pub operation: Operation,
    pub pwd: PathBuf,
    pub config: PathBuf,
}

impl TryFrom<Options> for Config {
    type Error = anyhow::Error;

    fn try_from(value: Options) -> Result<Self> {
        let operation = value.args.try_into()?;
        let config = get_config(value.configs)?;
        let pwd = get_pwd(value.pwd)?;

        return Ok(Config{ operation, config, pwd})
    }
}

fn get_config(config: Option<PathBuf>) -> Result<PathBuf> {
    if let Some(v) = config {
        return Ok(v);
    }

    if let Ok(loc) = std::env::var("XDG_CONFIG_HOME") {
        let mut loc = PathBuf::from(loc);
        loc.push("projector");
        loc.push("projector.json");
        return Ok(loc);
    }

    if let Ok(loc) = std::env::var("HOME") {
        let mut loc = PathBuf::from(loc);
        loc.push("projector");
        loc.push("projector.json");
        return Ok(loc);
    }
    
    return Err(anyhow!("Unable to find location"))
}

fn get_pwd(pwd: Option<PathBuf>) -> Result <PathBuf> {
    if let Some(v) = pwd {
        return Ok(v);
    }
    return Ok(std::env::current_dir().context("Error geting current directory")?);
}


#[cfg(test)]
mod test {
    use::anyhow::Result; 
    use crate::Options;
    use crate::Config;
    use crate::utils::config::Operation;

    #[test]
    fn test_print_all() -> Result<()> {
        let opts: Config = Options {
            args: vec![],
            pwd: None,
            configs: None,
        }.try_into()?;

        assert_eq!(opts.operation, Operation::Print(None) );
        return Ok(())
    }

    #[test]
    fn test_print_key() -> Result<()> {
        let opts: Config = Options {
            args: vec![
                String::from("foo"),
            ],
            pwd: None,
            configs: None,
        }.try_into()?;

        assert_eq!(opts.operation, Operation::Print(Some(String::from("foo"))) );
        return Ok(())
    }
 
    #[test]
    fn test_add_key_value() -> Result<()> {
        let opts: Config = Options {
            args: vec![
                String::from("add"),
                String::from("foo"),
                String::from("bar"),
            ],
            pwd: None,
            configs: None,
        }.try_into()?;

        assert_eq!(opts.operation, Operation::Add(
                String::from("foo"),
                String::from("bar"),
        ));
        return Ok(())
    }
    
    #[test]
    fn test_remove_key() -> Result<()> {
        let opts: Config = Options {
            args: vec![
                String::from("remove"),
                String::from("foo"),
            ],
            pwd: None,
            configs: None,
        }.try_into()?;

        assert_eq!(opts.operation, Operation::Remove(
                String::from("foo"),
        ));
        return Ok(())
    }

}
