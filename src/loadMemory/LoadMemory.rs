use crate::{parseMarkdown::ParseMarkdown, CustomError, CustomErrorStage, RenderEnv};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use walkdir::WalkDir;

//Functions in this files are among the costliest functions in the project and
//definetly need a refactor in the near future

use log::info;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Discovered {
    //File path is the key and document matter is the value
    pub data: std::collections::HashMap<String, ParseMarkdown::ContentDocument>,
}

impl Default for Discovered {
    fn default() -> Self {
        return Discovered {
            data: std::collections::HashMap::new(),
        };
    }
}

pub fn discover_content(
    local_render_env: &RenderEnv,
    content_full_data: &mut Discovered,
) -> Result<HashMap<String, HashMap<String, Vec<serde_yaml::value::Value>>>, CustomError> {
    let content_walker = WalkDir::new(&local_render_env.content_base);
    let mut building_forwardindex: HashMap<String, Vec<serde_yaml::value::Value>> = HashMap::new();
    let mut building_reverseindex: HashMap<String, HashMap<String, Vec<serde_yaml::value::Value>>> =
        HashMap::new();
    for i in content_walker.into_iter() {
        let entry = match i {
            Ok(entry) => entry,
            Err(e) => {
                return Err(CustomError {
                    stage: CustomErrorStage::StaticRender,
                    error: format!("[ERROR] Dir entry error : {}", e),
                })
            }
        };
        let path = entry.path();
        if path.is_file() {
            info!("Detected : {:?}", path);
            let content_store = match ParseMarkdown::parse(&path.display()) {
                Ok(content) => content.unwrap(), //unwrap is fine
                Err(e) => return Err(e),
            };
            BuildForwardIndex(&mut building_forwardindex, &content_store);
            BuildReverseIndex(&mut building_reverseindex, &content_store);
            content_full_data
                .data
                .insert(path.display().to_string(), content_store);
        }
    }
    MergeForwardIndex(content_full_data, building_forwardindex);
    //Forward index is merged with the frontmatter ,
    //Reverse index triggers a template to be rendered and only those templates
    //will get access to reverse index!
    Ok(building_reverseindex)
}

fn BuildForwardIndex(
    building_forwardindex: &mut HashMap<String, Vec<serde_yaml::value::Value>>,
    content_store: &crate::parseMarkdown::ParseMarkdown::ContentDocument,
) {
    match &content_store.frontmatter {
        Some(fmatter) => match fmatter.get("forwardindex") {
            Some(value) => match building_forwardindex.get_mut(value.as_str().unwrap()) {
                Some(r) => (*r).push(content_store.frontmatter.clone().unwrap()),
                None => {
                    let new_vec = vec![content_store.frontmatter.clone().unwrap()];
                    building_forwardindex.insert(value.as_str().unwrap().to_string(), new_vec);
                }
            },
            None => return,
        },
        None => return,
    }
}

fn BuildReverseIndex(
    building_reverseindex: &mut HashMap<String, HashMap<String, Vec<serde_yaml::value::Value>>>,
    content_store: &crate::parseMarkdown::ParseMarkdown::ContentDocument,
) {
    match &content_store.frontmatter {
        Some(fmatter) => match fmatter.get("reverseindex") {
            Some(value) => {
                //This value is the bunch of attributes to reverse merge on
                let mut attrs = Vec::new();
                if value.is_sequence() {
                    attrs.append(value.clone().as_sequence_mut().unwrap())
                } else if value.is_string() {
                    attrs.push(value.as_str().unwrap().into())
                }
                for i in attrs {
                    let frontmatter_clone = content_store.frontmatter.clone().unwrap();
                    let attr_value = frontmatter_clone.get(i.clone()).unwrap();
                    let mut attr_values = Vec::new();
                    if attr_value.is_sequence() {
                        attr_values.append(attr_value.clone().as_sequence_mut().unwrap());
                    } else {
                        attr_values.push(attr_value.clone());
                    }
                    for j in attr_values {
                        //These are gonna be the key in the hashmap
                        match building_reverseindex
                            .entry(i.as_str().unwrap().to_string())
                            .or_insert(HashMap::new())
                            .get_mut(j.as_str().unwrap())
                        {
                            Some(matter_array) => {
                                (*matter_array).push(frontmatter_clone.clone());
                            }
                            None => {
                                let new_arr = vec![frontmatter_clone.clone()];
                                building_reverseindex
                                    .get_mut(i.as_str().unwrap())
                                    .unwrap()
                                    .insert(j.as_str().unwrap().to_string(), new_arr);
                            }
                        }
                    }
                }
            }
            None => return,
        },
        None => return,
    }
}

fn MergeForwardIndex(
    content_full_data: &mut Discovered,
    building_forwardindex: HashMap<String, Vec<serde_yaml::value::Value>>,
) {
    let content_hashmap_copy = content_full_data.clone();
    let keys = content_hashmap_copy.data.keys();
    for k in keys {
        let mutvalueref = content_full_data.data.get_mut(k).unwrap();
        //[TODO]change all these ownded values to refrences, need to check render methods
        //compatibility for it
        (*mutvalueref).forwardindex = Some(building_forwardindex.clone());
    }
}
