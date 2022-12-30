use comrak::{format_html, parse_document, Arena, ComrakOptions};
use std::fs;

pub fn parse() {
    let mut options = ComrakOptions::default();
    options.extension.front_matter_delimiter = Some("---".to_owned());
    let md = fs::read_to_string("./content/home.md").unwrap();
    let arena = Arena::new();
    let root = parse_document(
        &arena,
        &md,
        &options,
    );
    //frontmatter is either the first child in the ast or its not present anywhere!
    let frontmatter = match root.children().nth(0){
        Some(matter) => {
            match matter {
                comrak::nodes::AstNode::from(comrak::NodeValue::FrontMatter)=> { Some(matter) }
                _ => { None }
            }
        },
        _ => { None }

    };
}
