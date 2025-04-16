use serde_yaml::Value;
use std::collections::HashMap;
use std::fs;

pub fn load_yaml_from_file(yaml_path: &str) -> HashMap<String, Value> {
    let string_yaml = match fs::read_to_string(yaml_path) {
        Ok(s) => s,
        Err(e) => {
            log::error!("Error reading yaml settings file : {} ", e);
            log::info!("Continuing with defaults");
            return HashMap::new();
        }
    };

    let settings: HashMap<String, Value> =
        match serde_yaml::from_str::<HashMap<String, Value>>(string_yaml.as_str()) {
            Ok(s) => s,
            Err(e) => {
                log::error!("Error parsing settings yaml file : {}", e);
                log::info!("Continuing with defaults...");
                return HashMap::new();
            }
        };
    return settings;
}

pub fn get_inner_value<T: serde::de::DeserializeOwned>(
    serde_map: &HashMap<String, Value>,
    deref_vec: Vec<String>,
    default_value_when_anything_not_found: T,
) -> T {
    let mut last_value: &Value = &Value::Null;
    for (i, v) in deref_vec.iter().enumerate() {
        log::info!("derefrencing : {}", v);
        if i == 0 {
            last_value = match serde_map.get(v) {
                Some(s) => s,
                None => {
                    log::error!("Error derefrencing in settings yaml!");
                    return default_value_when_anything_not_found;
                }
            }
        } else {
            last_value = match last_value.get(v) {
                Some(s) => {
                    if i == deref_vec.len()-1 {
                        return match serde_yaml::from_value(s.to_owned()) {
                            Ok(v) => v,
                            Err(e) => {
                                log::error!("Error converting from Value to concrete type : {}", e);
                                return default_value_when_anything_not_found;
                            }
                        };
                    }
                    s
                }
                None => {
                    log::error!("Error derefrencing in settings yaml!");
                    return default_value_when_anything_not_found;
                }
            }
        }
    }
    return default_value_when_anything_not_found;
}
