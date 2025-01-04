use std::{collections::HashMap, path::PathBuf};
use serde::{Serialize, Deserialize};
use std::fs;

use crate::utils::config::Config;


#[derive(Debug, Serialize, Deserialize)]
struct Data {
    pub projector: HashMap<PathBuf, HashMap<String, String>>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Projector {
    config: Config,
    data: Data,
}

// const DEFAULT_DATA: Data = Data{ projector: HashMap::new() };

fn get_default_data() -> Data {
    return Data{ projector: HashMap::new() };

}

impl Projector {
    pub fn get_value(&self, key: &str) -> Option<&String> {
        let mut curr = Some(self.config.pwd.as_path());
        let mut out = None;

        while let Some(p) = curr {
            if let Some(dir) = self.data.projector.get(p) {
                if let Some(value) = dir.get(key) {
                    out = Some(value);
                    break;
                } 
            }
            curr = p.parent()
        }
        return out;
    }
    

    pub fn from_config(config: Config) -> Self {
        if std::fs::metadata(&config.config).is_ok() {
            let contents = fs::read_to_string(&config.config);
            let contents = contents.unwrap_or(
                String::from("{\"projector\":{}}")
            );

            let data = serde_json::from_str(&contents);
            let data = data.unwrap_or(get_default_data());

            return Projector {
                config, data,
            };
        }
        return Projector {
            config, data: get_default_data()
        }
    }


    pub fn get_value_all(&self) -> HashMap<&String, &String>{
        let mut curr = Some(self.config.pwd.as_path());
        let mut paths = vec ![];

        while let Some(p) = curr {
            paths.push(p);
            curr = p.parent()
        }

        let mut out = HashMap::new();
        for path in paths.into_iter().rev() {
            if let Some(map) = self.data.projector.get(path) {
                out.extend(map.iter())
            }
        }

        return out;
    }

    pub fn set_value(&mut self, key: String, value: String) {
        self.data.projector.entry(self.config.pwd.clone()).or_default().insert(key, value);
    }

    pub fn delete_value(&mut self, key: &String) {
        self.data.projector.entry(self.config.pwd.clone()).or_default().remove(key);
    }

}





#[cfg(test)]
mod test {
    use std::{collections::HashMap, path::PathBuf};

    use collection_macros::hashmap;

    use crate::utils::config::{Config, Operation};

    use super::{Data, Projector};

    fn get_data() -> HashMap<PathBuf, HashMap<String,String>> {
        return hashmap! {
                PathBuf::from("/") => hashmap! {
                    "foo".into() => "bar1".into(),
                    "bar".into() => "bazz".into(),
                },
                PathBuf::from("/foo") => hashmap! {
                    "foo".into() => "bar2".into()
                },
                PathBuf::from("/foo/bar") => hashmap! {
                    "foo".into() => "bar3".into(),
                },
                PathBuf::from("/foo/bar/baz") => hashmap! {
                    "foo".into() => "bar3".into()
                },
            }
        }

    
    fn get_projector(pwd: PathBuf) -> Projector {
        return Projector {
            config: Config {
                pwd,
                config: PathBuf::from(""),
                operation: Operation::Print(None),
            },
            data: Data{
                projector: get_data(),
            }
        };
    }


    #[test]
    fn get_value() {
        let mut proj = get_projector(PathBuf::from("/foo/bar"));
        assert_eq!(proj.get_value("foo"), Some(&String::from("bar3")));
        proj.set_value(String::from("fem"), String::from("looking great"));
        assert_eq!(proj.get_value("fem"), Some(&String::from("looking great")));
    }

    #[test]
    fn remove_value() {
        let mut proj = get_projector(PathBuf::from("/foo/bar"));
        proj.delete_value(&String::from("foo"));
        assert_eq!(proj.get_value("foo"), Some(&String::from("bar2")));
    }
}
