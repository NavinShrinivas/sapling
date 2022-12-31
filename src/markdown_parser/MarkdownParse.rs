use comrak::{format_html, nodes::NodeValue, parse_document, Arena, ComrakOptions};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs, path::Path};

#[derive(Serialize, Deserialize, Debug)]
pub struct ContentDocument {
    pub frontmatter_raw: Option<String>,
    pub frontmatter: Option<serde_yaml::Value>,
    pub content: Option<String>,
    pub name: Option<String>,
}

impl ContentDocument {
    fn new(file_name: &str) -> ContentDocument {
        ContentDocument {
            frontmatter_raw: None,
            frontmatter: None,
            content: None,
            name: Some(file_name.to_string()),
        }
    }
}

pub fn parse<S: std::string::ToString>(md_file_name: S) -> Option<ContentDocument> {
    let mut options = ComrakOptions::default();
    options.extension.front_matter_delimiter = Some("---".to_owned());
    options.extension.strikethrough = true;
    options.extension.tagfilter = true;
    options.extension.table = true;
    options.extension.autolink = true;
    options.extension.tasklist = true;
    options.extension.superscript = true;
    options.extension.footnotes = true;
    let md = match fs::read_to_string(md_file_name.to_string()) {
        Ok(fd) => fd,
        _ => return None,
    };
    let mut content_doc = ContentDocument::new(
        Path::new(&md_file_name.to_string())
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .trim_end_matches(".md"),
    );

    let arena = Arena::new();
    let root = parse_document(&arena, &md, &options);
    //frontmatter is either the first child in the ast or its not present anywhere!
    let frontmatter = match root.children().nth(0) {
        Some(matter) => match matter.data.borrow().value.clone() {
            NodeValue::FrontMatter(text_vec) => {
                let matter_string = match String::from_utf8(text_vec) {
                    Ok(s) => s,
                    _ => {
                        panic!(
                            "Error parsing frontmatter in : {}",
                            md_file_name.to_string()
                        )
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
            _ => None,
        },
        _ => None,
    };
    content_doc.frontmatter_raw = frontmatter;
    content_doc.frontmatter =
        serde_yaml::from_str(content_doc.frontmatter_raw.as_ref().unwrap()).unwrap();
    let mut html = vec![];
    format_html(root, &options, &mut html).unwrap();
    content_doc.content = Some(String::from_utf8(html).unwrap());
    Some(content_doc)
}
