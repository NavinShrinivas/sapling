#[allow(dead_code)]
#[allow(non_snake_case)]

use std::{path::PathBuf, rc::Rc};
use tera::Context;
use walkdir::WalkDir;

use crate::markdown_parser::MarkdownParse;
use crate::RenderEnv;
use crate::TemplatesMetaData;

#[derive(Debug)]
pub enum CustomErrorType {
    StaticRender,
}
#[derive(Debug)]
pub struct CustomError {
    r#type: CustomErrorType,
    error: String,
}

fn get_rel_path_in_folder(path: &PathBuf) -> String {
    let mut path_iter = path.parent().unwrap().iter();
    path_iter.next(); //To skip the base dir
    let mut answer = String::new();
    for i in path_iter {
        answer = format!("{}/{}", answer, i.to_str().unwrap());
    }
    return answer;
}

fn validate_template_request(
    frontmatter: &serde_yaml::Value,
    local_render_env: Rc<RenderEnv>,
    template_meta: &TemplatesMetaData,
) -> Result<String, CustomError> {
    let requested_template = match frontmatter.get("template") {
        Some(template_path) => template_path.as_str().unwrap(),
        _ => {
            println!("\t No templates property found in frontmatter, defaulting.");
            &local_render_env.default_template
        }
    };
    let parsed_templates: Vec<_> = template_meta
        .compiled_tera_instance
        .get_template_names()
        .collect();
    if parsed_templates.contains(&requested_template) {
        return Ok(requested_template.to_string());
    } else {
        return Err(CustomError {
            r#type: CustomErrorType::StaticRender,
            error: String::from("Error finding a proper template to render markdown."),
        });
    }
}

pub fn static_render(
    local_render_env: Rc<RenderEnv>,
    template_meta: &TemplatesMetaData,
) -> Result<(), CustomError> {
    match std::fs::remove_dir_all(&local_render_env.static_base) {
        Ok(_) => {
            println!("Reusing previous builds like cache not yet possible, rebuilding from scratch.");
            //[TODO] use cached static sites
        }
        _ => {
            println!(
                "Static dir not found, building from scratch! Reusing builds not yet possible."
            );
        }
    }
    std::fs::create_dir(&local_render_env.static_base).unwrap();
    let content_walker = WalkDir::new(&local_render_env.content_base);
    for i in content_walker.into_iter() {
        let path = i.unwrap().into_path();
        let rel_path = get_rel_path_in_folder(&path);
        if path.is_file() {
            println!("rendering : {:?}", path);
            let content_store = MarkdownParse::parse(path.clone().display()).unwrap();
            let frontmatter = content_store.frontmatter.as_ref().unwrap();
            match validate_template_request(
                frontmatter,
                Rc::clone(&local_render_env),
                &template_meta,
            ) {
                Ok(template_to_use) => {
                    println!("\ttemplate : {}", template_to_use);
                    let static_store = template_meta
                        .compiled_tera_instance
                        .render(
                            &template_to_use,
                            &Context::from_serialize(&content_store).unwrap(),
                        )
                        .unwrap();
                    let write_path = format!(
                        "{}{}/{}.html",
                        &local_render_env.static_base,
                        rel_path,
                        content_store.name.as_ref().unwrap()
                    );
                    println!("\trendering to : {}", write_path);
                    match std::fs::create_dir(format!(
                        "{}{}",
                        &local_render_env.static_base, rel_path
                    )) {
                        Ok(_) => {}
                        _ => {} //We don't want to do anything if the folder exists, and nothing to do if
                                //we created it just now.
                    }
                    std::fs::write(write_path, static_store).unwrap();
                }
                Err(e) => return Err(e),
            }
        }
    }
    Ok(())
}
