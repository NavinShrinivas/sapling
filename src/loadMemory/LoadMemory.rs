use crate::{parseMarkdown::ParseMarkdown, CustomError, CustomErrorStage, RenderEnv};
use futures::future::join_all;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use walkdir::WalkDir;
use crate::parseMarkdown::ParseMarkdown::MDRenderOptions;

//Functions in this files are among the costliest functions in the project and
//definetly need a refactor in the near future

use log::info;
use std::sync::{Arc, Mutex, RwLock};

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

pub async fn discover_content(
    md_render_config: MDRenderOptions,
    local_render_env: &RenderEnv,
    content_full_data: &mut Discovered,
) -> Result<Arc<RwLock<HashMap<String, HashMap<String, Vec<serde_yaml::value::Value>>>>>, CustomError>
{
    let content_walker = WalkDir::new(&local_render_env.content_base);
    let building_forwardindex: Arc<RwLock<HashMap<String, Vec<serde_yaml::value::Value>>>> =
        Arc::new(RwLock::new(HashMap::new()));
    let building_reverseindex: Arc<
        RwLock<HashMap<String, HashMap<String, Vec<serde_yaml::value::Value>>>>,
    > = Arc::new(RwLock::new(HashMap::new()));

    let content_document_map: Arc<Mutex<HashMap<String, ParseMarkdown::ContentDocument>>> =
        Arc::new(Mutex::new(HashMap::new()));

    let md_render_config = Arc::new(md_render_config);

    let mut handles: Vec<_> = Vec::new();
    let lock = Arc::new(Mutex::new(0));
    for i in content_walker.into_iter() {
        let local_lock: Arc<Mutex<u32>> = lock.clone();
        let entry = match i {
            Ok(entry) => entry,
            Err(e) => {
                return Err(CustomError {
                    stage: CustomErrorStage::StaticRender,
                    error: format!("[ERROR] Dir entry error : {}", e),
                });
            }
        };
        info!("Detected : {:?}", entry.path());
        let local_fi = Arc::clone(&building_forwardindex);
        let local_ri = Arc::clone(&building_reverseindex);
        let local_cdm = Arc::clone(&content_document_map);
        let local_md_render_config = Arc::clone(&md_render_config);

        if entry.path().is_file() {
            let job = tokio::spawn(async move {
                let path = entry.path();
                let content_store = match ParseMarkdown::parse(&local_md_render_config, &path.display()) {
                    Ok(content) => content.unwrap(), //unwrap is fine
                    Err(e) => return Err(e),
                };
                //As we are using a parent lock, cotention on hasmap should be low
                let _gaurd = local_lock
                    .lock()
                    .expect("Something went wrong getting outer lock");
                BuildForwardIndex(local_fi, &content_store);
                BuildReverseIndex(local_ri, &content_store);
                drop(_gaurd);
                //Lock drops here
                local_cdm
                    .lock()
                    .unwrap()
                    .insert(path.display().to_string(), content_store);
                Ok("All good!")
            });
            handles.push(job);
        }
    }
    join_all(handles).await;
    for (k, v) in content_document_map.lock().unwrap().iter() {
        content_full_data.data.insert(k.clone(), v.clone());
    }
    MergeForwardIndex(content_full_data, building_forwardindex);
    //Forward index is merged with the frontmatter ,
    //Reverse index triggers a template to be rendered and only those templates
    //will get access to reverse index!
    Ok(building_reverseindex)
}

fn BuildForwardIndex(
    building_forwardindex: Arc<RwLock<HashMap<String, Vec<serde_yaml::value::Value>>>>,
    content_store: &crate::parseMarkdown::ParseMarkdown::ContentDocument,
) {
    match &content_store.frontmatter {
        Some(fmatter) => match fmatter.get("forwardindex") {
            Some(value) => {
                //Forwardindex should be an array, allowing mapping frontmatters on multiple keys!
                match value.as_sequence() {
                    Some(forward_index_key) => {
                        for inner_value in forward_index_key.iter() {
                            //We are usin RW locks, so we must take write...only once in one
                            //lifetime
                            building_forwardindex
                                .write()
                                .expect("Poisoned Hasmap Lock")
                                .entry(inner_value.as_str().unwrap().to_string())
                                .and_modify(|v| v.push(content_store.frontmatter.clone().unwrap()))
                                .or_insert(vec![content_store.frontmatter.clone().unwrap()]);
                        }
                    }
                    None => {
                        building_forwardindex
                            .write()
                            .expect("Poisoned Hasmap Lock")
                            .entry(value.as_str().unwrap().to_string())
                            .and_modify(|v| v.push(content_store.frontmatter.clone().unwrap()))
                            .or_insert(vec![content_store.frontmatter.clone().unwrap()]);
                    } 
                }
            }
            None => return,
        },
        None => return,
    }
}

fn BuildReverseIndex(
    building_reverseindex: Arc<
        RwLock<HashMap<String, HashMap<String, Vec<serde_yaml::value::Value>>>>,
    >,
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
                        let inner_fm = frontmatter_clone.clone();

                        building_reverseindex
                            .write()
                            .expect("Poisoned hashmap lock")
                            .entry(i.as_str().unwrap().to_string())
                            .and_modify(|inner_map| {
                                inner_map
                                    .entry(j.as_str().unwrap().to_string())
                                    .and_modify(|array| array.push(frontmatter_clone.clone()))
                                    .or_insert(vec![frontmatter_clone.clone()]);
                            })
                            .or_insert(
                                HashMap::from([(j.as_str().unwrap().to_string(), vec![inner_fm])])
                            );
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
    building_forwardindex: Arc<RwLock<HashMap<String, Vec<serde_yaml::value::Value>>>>,
) {
    let content_hashmap_copy = content_full_data.clone();
    let keys = content_hashmap_copy.data.keys();
    for k in keys {
        let mutvalueref = content_full_data.data.get_mut(k).unwrap();
        //[TODO]change all these ownded values to refrences, need to check render methods
        //compatibility for it
        (*mutvalueref).forwardindex = Some(building_forwardindex.read().unwrap().to_owned());
    }
}
