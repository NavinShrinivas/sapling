#[allow(dead_code)]
#[allow(non_snake_case)]
use crate::{CustomError, CustomErrorStage};
use comrak::{format_html, nodes::NodeValue, parse_document, Arena, ComrakOptions};
use log::warn;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs, path::Path};
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ContentDocument {
    pub frontmatter_raw: Option<String>,
    pub frontmatter: Option<serde_yaml::value::Value>,
    pub content: Option<String>,
    pub name: Option<String>,
    pub forwardindex: Option<HashMap<String, Vec<serde_yaml::value::Value>>>,
}

impl ContentDocument {
    fn new(file_name: &str) -> ContentDocument {
        ContentDocument {
            frontmatter_raw: None,
            frontmatter: None,
            content: None,
            name: Some(file_name.to_string()),
            forwardindex: Some(HashMap::new()),
        }
    }
}

//Represents the Markdown render options in the config file
#[derive(Serialize, Deserialize, Debug)]
pub struct MDRenderOptions {
    pub unsafe_render: bool,
}

impl Default for MDRenderOptions {
    fn default() -> Self {
        MDRenderOptions {
            unsafe_render: false,
        }
    }
}


pub fn parse<S: std::string::ToString>(
    md_render_config : &MDRenderOptions,
    md_file_path: &S,
) -> Result<Option<ContentDocument>, CustomError> {
    let mut options = ComrakOptions::default();
    options.extension.front_matter_delimiter = Some("---".to_owned());
    options.extension.strikethrough = true;
    options.extension.table = true;
    options.extension.tasklist = true;
    options.render.unsafe_ = md_render_config.unsafe_render;

    // options.extension.autolink = true;
    // options.extension.tagfilter = true;
    // options.extension.superscript = true;
    // options.extension.footnotes = true;
    let md = match fs::read_to_string(md_file_path.to_string()) {
        Ok(fd) => fd,
        Err(e) => {
            return Err(CustomError {
                stage: CustomErrorStage::ParseMarkdown,
                error: format!("[ERROR] Couldn't read markdown files : {}", e),
            })
        }
    };
    let file_path = md_file_path.to_string();
    let md_file_name = match Path::new(&file_path).file_name() {
        Some(file_name) => {
            file_name.to_str().unwrap().trim_end_matches(".md") // !allow[unwrap]
        }
        None => {
            return Err(CustomError {
                stage: CustomErrorStage::ParseMarkdown,
                error: format!(
                    "[ERROR] Couldn't find markdown files name from path : {}",
                    md_file_path.to_string()
                ),
            })
        }
    };
    let mut content_doc = ContentDocument::new(md_file_name);

    let arena = Arena::new();
    let root = parse_document(&arena, &md, &options);
    //frontmatter is either the first child in the ast or its not present anywhere!
    let frontmatter = match root.children().nth(0) {
        Some(matter) => match &matter.data.borrow().value {
            //RefCells need borrow
            NodeValue::FrontMatter(text_vec) => {
                let matter_string = match String::from_utf8(text_vec.to_vec()) {
                    Ok(s) => s,
                    Err(e) => {
                        return Err(CustomError {
                            stage: CustomErrorStage::ParseMarkdown,
                            error: format!(
                                "[ERROR] error parsing frontmatter in {} : {}",
                                md_file_path.to_string(),
                                e
                            ),
                        })
                    }
                };
                Some(
                    matter_string
                        .trim()
                        .trim_end_matches("-")
                        .trim_start_matches("-")
                        .to_string(),
                )
            }
            _ => {
                warn!("No frontmatter found for : {}", md_file_path.to_string());
                None
            }
        },
        _ => {
            warn!("Empty markdown file found : {}", md_file_path.to_string());
            None
        }
    };
    let frontmatter = match frontmatter {
        None => {
            let default_frontmatter = "template:index.html\n";
            default_frontmatter.to_string()
        }
        Some(unwrap_frontmatter) => unwrap_frontmatter,
    };
    content_doc.frontmatter_raw = Some(frontmatter);
    content_doc.frontmatter =
        match serde_yaml::from_str(&content_doc.frontmatter_raw.as_ref().unwrap()) {
            //Above unwrap is fine
            Ok(hashmap) => hashmap,
            Err(e) => {
                return Err(CustomError {
                    stage: CustomErrorStage::ParseMarkdown,
                    error: format!("[ERROR] error parsing yaml : {}", e),
                })
            }
        };

    //Some default and compulsory dependant fields :
    content_doc.name = match content_doc.frontmatter.as_ref().unwrap().get("name") {
        //This unwwap
        //is fine
        Some(name) => Some(name.as_str().unwrap().to_string()), //[refactor]
        _ => Some(md_file_name.to_string()),
    };
    //templates default is handled in seperate places

    //parsing to html
    let mut html = vec![];
    match format_html(root, &options, &mut html) {
        Ok(_) => {}
        Err(e) => {
            return Err(CustomError {
                stage: CustomErrorStage::ParseMarkdown,
                error: format!(
                    "[ERROR] Error parsing html from markdown in file {} : {}",
                    md_file_path.to_string(),
                    e
                ),
            })
        }
    };
    content_doc.content = Some(match String::from_utf8(html){
        Ok(v) => v,
        Err(_) => {
            return Err(CustomError{
                stage : CustomErrorStage::ParseMarkdown,
                error : format!("[ERROR] Error formning string from parsed html vector, possibly bad encoding or illegal charecters. Please stick to utf-8")
            })
        }
    });
    Ok(Some(content_doc))
}
