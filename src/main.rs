mod markdown_parser;

use std::{collections::HashMap, rc::Rc};
use tera::{Context, Tera};
use walkdir::WalkDir;

use markdown_parser::MarkdownParse;

#[derive(Debug)]
struct TemplatesMetaData {
    relative_paths: Vec<String>, //Paths of templates relative to `template_base` folder
    compiled_tera_instance: Tera,
}
impl TemplatesMetaData {
    fn new(local_render_env: Rc<RenderEnv>) -> Result<TemplatesMetaData, tera::Error> {
        let tera_instance = match Tera::new(&format!("{}/**/*", local_render_env.template_base)) {
            Ok(tera) => tera,
            Err(e) => return Err(e),
        };
        Ok(TemplatesMetaData {
            relative_paths: Vec::new(),
            compiled_tera_instance: tera_instance,
        })
    }
}

fn discover_templates(
    local_render_env: Rc<RenderEnv>,
    template_meta: &mut TemplatesMetaData,
) -> Result<String, walkdir::Error> {
    for i in WalkDir::new(&local_render_env.template_base) {
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
        println!("\t{}", entry.path().display().to_string());
        template_meta
            .relative_paths
            .push(entry.path().display().to_string());
    }
    Ok(String::from(
        "Successfully discovered all templates from given base dir.",
    ))
}

#[derive(Debug)]
enum CustomErrorType {
    StaticRender,
}

#[derive(Debug)]
struct CustomError {
    r#type: CustomErrorType,
    error: String,
}

fn static_render(
    local_render_env: Rc<RenderEnv>,
    template_meta: &TemplatesMetaData,
) -> Result<(), CustomError> {
    match std::fs::remove_dir_all(&local_render_env.static_base) {
        Ok(_) => {}
        _ => {
            println!("Static dir not found, building from scratch!");
            //[TODO] use cached static sites
        }
    }
    std::fs::create_dir(&local_render_env.static_base).unwrap();
    let content_walker = WalkDir::new(&local_render_env.content_base);
    for i in content_walker.into_iter() {
        let path = i.unwrap().into_path();
        if path.is_file() {
            let content_store = MarkdownParse::parse(path.clone().display()).unwrap();
            println!("rendering : {:?}", path);
            let frontmatter = content_store.frontmatter.as_ref().unwrap();
            let requested_template = match frontmatter.get("template") {
                Some(template_path) => template_path.as_str().unwrap(),
                _ => {
                    println!("\t No templates proerpty found in frontmatter, defaulting.");
                    &local_render_env.default_template
                }
            };
            let parsed_teamplates: Vec<_> = template_meta
                .compiled_tera_instance
                .get_template_names()
                .collect();
            if parsed_teamplates.contains(&requested_template) {
                println!("\t template : {}", requested_template);
                let static_store = template_meta
                    .compiled_tera_instance
                    .render(
                        requested_template,
                        &Context::from_serialize(&content_store).unwrap(),
                    )
                    .unwrap();
                std::fs::write(
                    format!(
                        "{}/{}.html",
                        &local_render_env.static_base,
                        content_store.name.unwrap()
                    ),
                    static_store,
                )
                .unwrap();
            }
        }
    }
    // Deserialize it back to a Rust type.

    Ok(())
}

struct RenderEnv {
    template_base: String,
    content_base: String,
    static_base: String,
    default_template: String,
}

impl RenderEnv {
    fn new<S: std::string::ToString>(
        template_base: S,
        content_base: S,
        static_base: S,
        default_template: S,
    ) -> RenderEnv {
        RenderEnv {
            template_base: template_base.to_string(),
            content_base: content_base.to_string(),
            static_base: static_base.to_string(),
            default_template: default_template.to_string(),
        }
    }
}

fn main() {
    // let template_base = "templates"; //[TODO] make this a command line arg, [TODO] be able to
    //                                  //handle any type of path
    // let content_base = "content";
    // let static_base = "static";
    // let default_template = "index.html";

    let global_render_env = Rc::new(RenderEnv::new(
        "templates",
        "content",
        "static",
        "index.html",
    ));

    let local_render_env = Rc::clone(&global_render_env);

    //Using walkdir for metadata, but using globs for tera
    //This is because, Tera can do inherited templates only if we use its blob function
    let mut template_meta = match TemplatesMetaData::new(local_render_env) {
        Ok(s) => {
            println!("All detected templates parsed without errors!");
            s
        }
        Err(e) => {
            panic!("{}", e)
        }
    };
    println!("Templates detected : ");
    match discover_templates(Rc::clone(&global_render_env), &mut template_meta) {
        Ok(res) => {
            println!("{}", res);
        }
        Err(e) => {
            panic!("{:?}", e)
        }
    }
    static_render(Rc::clone(&global_render_env), &template_meta).unwrap();
}
