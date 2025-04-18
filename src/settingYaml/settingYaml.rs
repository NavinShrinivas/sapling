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
    if deref_vec.len() == 0 {
        log::error!("Error derefrencing in settings yaml, empty deref vector!");
        return default_value_when_anything_not_found;
    }
    if deref_vec.len() == 1 {
        match serde_map.get(deref_vec[0].as_str()) {
            Some(s) => {
                return match serde_yaml::from_value(s.to_owned()) {
                    Ok(v) => v,
                    Err(e) => {
                        log::error!("Error converting from Value to concrete type : {}", e);
                        panic!()
                    }
                };
            }
            None => {
                log::error!("Error derefrencing in settings yaml, couldnt find the said key!");
                return default_value_when_anything_not_found;
            }
        }
    }
    for (i, v) in deref_vec.iter().enumerate() {
        log::info!("derefrencing : {}", v);
        if i == 0 {
            last_value = match serde_map.get(v) {
                Some(s) => s,
                None => {
                    log::error!("Error derefrencing in settings yaml, couldnt find the said key!");
                    return default_value_when_anything_not_found;
                }
            }
        } else {
            last_value = match last_value.get(v) {
                Some(s) => {
                    if i == deref_vec.len() - 1 {
                        return match serde_yaml::from_value(s.to_owned()) {
                            Ok(v) => v,
                            Err(e) => {
                                log::error!("Error converting from Value to concrete type : {}", e);
                                panic!()
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

pub fn search_for_item_in_vec_and_return_value(
    vec: &Vec<Value>,
    item: &str,
    value: &str,
) -> Option<Value> {
    for i in vec.iter() {
        match i.get(item) {
            Some(v) => {
                if v.as_str().is_some() && v.as_str().unwrap() == value {
                    return Some(i.clone());
                }
            }
            None => {}
        }
    }
    None
}

pub fn get_value_from_value(
    value: &Value,
    key: &str,
) -> Option<Value> {
    match value.get(key) {
        Some(v) => Some(v.clone()),
        None => {
            log::error!("Error getting {} from value", key);
            None
        },
    }
}
