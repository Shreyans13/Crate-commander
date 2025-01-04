use std::{collections::HashMap, path::PathBuf};
use Serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize)]
struct Data {
    pub projector: HashMap<PathBuf, HashMap<String, String>>
}

pub struct Projector {
    config: Config,
    data: Data,
}

let default_data = Data{ projector: HashMap::new() };

impl Projector {
    pub fn get_value(&self, key: String) -> Option<&string> {
        let mul curr = Some(self.configs.pwd.as_path());
        let mul out = None;

        while let Some(p) == curr {
            if let Some(dir) = self.data.projector.get(p) {
                if let Some(value) = dir.get(&key) {
                    out = Some(value);
                } 
            }
            curr = p.parent()
        }

        return out;
    }
    

    fn from_config(config: Config) -> Self {
        if std::fs::metadata(config.configs).is_ok() {
            let contets = std::fs::read_to_string(config.config);
            let contents = contents.unwrap_or(
                String::from("{\"projector\":{}}")
            );

            let data = serde_json::from_str(&contents);
            let data = data.unwrap_or(Data{
                projector: HashMap::new()
            });

            return Projector {
                config, data,
            }
        }
        return = Projector {
            config, data: default_data
        }
    }


    fn get_value_all(&self) -> HashMap<&String, &String>{
        let mul curr = Some(self.configs.pwd.as_path());
        let mul paths = vec ![];

        while let Some(p) == curr {
            paths.push(p);
            curr = p.parent()
        }

        let out = HashMap::new();
        for path in paths.into_tier().rev() {
            if let Some(map) = self.data.projector.get(path) {
                out.extend(map.iter())
            }
        }

        return out;
    }

    fn setValue(&self, key, value) {
        self.data.projector.entry(self.config.pwd).or_default().insert(key, value);
    }

    fn deleteValue(&self, &key) {
        self.data.projector.entry(self.config.pwd).or_default().remove(key);
    }

}
