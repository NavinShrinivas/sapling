use crate::{markdown_parser::MarkdownParse, CustomError, CustomErrorStage, Discovered, RenderEnv};
use std::collections::HashMap;
use walkdir::WalkDir;

pub fn discover_content(
    local_render_env: &RenderEnv,
    content_full_data: &mut Discovered,
) -> Result<(), CustomError> {
    let content_walker = WalkDir::new(&local_render_env.content_base);
    let mut building_forwardindex: HashMap<String, Vec<serde_yaml::value::Value>> = HashMap::new();
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
            println!("[INFO] Detected : {:?}", path);
            let content_store = match MarkdownParse::parse(&path.display()) {
                Ok(content) => content.unwrap(), //unwrap is fine
                Err(e) => return Err(e),
            };
            BuildForwardIndex(&mut building_forwardindex, &content_store);
            content_full_data
                .data
                .insert(path.display().to_string(), content_store);
        }
    }
    MergeForwardIndex(content_full_data, building_forwardindex);
    Ok(())
}

fn BuildForwardIndex(
    building_forwardindex: &mut HashMap<String, Vec<serde_yaml::value::Value>>,
    content_store: &crate::markdown_parser::MarkdownParse::ContentDocument,
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

fn MergeForwardIndex(
    content_full_data: &mut Discovered,
    building_forwardindex: HashMap<String, Vec<serde_yaml::value::Value>>,
) {
    let content_hashmap_copy = content_full_data.clone();
    let keys  = content_hashmap_copy.data.keys();
    for k in keys{
        let mutvalueref = content_full_data.data.get_mut(k).unwrap();
        //[TODO]change all these ownded values to refrences, need to check render methods
        //compatibility for it
        (*mutvalueref).forwardindex= Some(building_forwardindex.clone());
    }
}
