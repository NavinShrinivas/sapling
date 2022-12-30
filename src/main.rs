mod markdown_parser;


use tera::{Tera, Context};
use walkdir::WalkDir;
use std::collections::HashMap;

use markdown_parser::parse_functions;

#[derive(Debug)]
struct TemplatesMetaData {
    relative_paths: Vec<String>, //Paths of templates relative to `template_base` folder
    compiled_tera_instance: Tera,
}
impl TemplatesMetaData {
    fn new<T: std::string::ToString + std::fmt::Display>(
        template_base: T,
    ) -> Result<TemplatesMetaData, tera::Error> {
        let tera_instance = match Tera::new(&format!("{}/**/*", template_base)) {
            Ok(tera) => tera,
            Err(e) => return Err(e),
        };
        Ok(TemplatesMetaData {
            relative_paths: Vec::new(),
            compiled_tera_instance: tera_instance,
        })
    }
}

fn discover_templates<S: std::string::ToString + AsRef<std::path::Path>>(
    template_meta: &mut TemplatesMetaData,
    template_base: S,
) -> Result<String, walkdir::Error> {
    for i in WalkDir::new(template_base) {
        let entry = match i {
            Ok(i) => {
                if i.file_type().is_dir() {
                    continue;
                } else {
                    i
                }
            }
            Err(e) => return Err(e),
        };
        println!("{}", entry.path().display().to_string());
        template_meta
            .relative_paths
            .push(entry.path().display().to_string());
    }
    Ok(String::from(
        "Successfully discovered all templates from given base dir.",
    ))
}


fn main() {
    let template_base = "templates"; //[TODO] make this a command line arg, [TODO] be able to
                                     //handle any type of path

    //Using walkdir for metadata, but using globs for tera
    //This is because, Tera can do inherited templates only if we use its blob function
    let mut template_meta = match TemplatesMetaData::new(template_base) {
        Ok(s) => {
            println!("All detected templates parsed without errors!");
            s
        },
        Err(e) => {
            panic!("{}", e)
        }
    };
    println!("Templates detected : ");
    match discover_templates(&mut template_meta, template_base) {
        Ok(res) => {
            println!("{}", res);
        }
        Err(e) => {
            panic!("{:?}", e)
        }
    }
    println!("{:#?}",template_meta);

    let yaml = "content: 'hello world'\ntitle: 'haii'\n";

    // Deserialize it back to a Rust type.
    let deserialized_map: HashMap<String, String> = serde_yaml::from_str(&yaml).unwrap();
    println!("{}",template_meta.compiled_tera_instance.render("blogs_templates/blog_basic_template.html",&Context::from_serialize(&deserialized_map).unwrap()).unwrap());
    parse_functions::parse();
}
