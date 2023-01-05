#[allow(dead_code)]
#[allow(non_snake_case)]
use crate::{CustomError, CustomErrorType};
use lightningcss::{
    bundler::{Bundler, FileProvider},
    printer::PrinterOptions,
    stylesheet::ParserOptions,
};
use std::path::Path;
use std::{path::PathBuf, rc::Rc};
use tera::Context;
use walkdir::WalkDir;

use crate::markdown_parser::MarkdownParse;
use crate::RenderEnv;
use crate::TemplatesMetaData;

fn decide_static_serve_path(
    local_render_env: Rc<RenderEnv>,
    content_store: &crate::markdown_parser::MarkdownParse::ContentDocument,
) -> String {
    //First check if the frontmatter has `link`
    match content_store.frontmatter.as_ref().unwrap().get("link") {
        Some(link) => {
            let link_str = link.as_str().unwrap().to_string();
            let clean_path = link_str
                .trim()
                .trim_end_matches("/")
                .trim_start_matches("/");
            let fqd = format!("{}/{}", local_render_env.static_base, clean_path);
            let fqp = format!("{}/{}/index.html", local_render_env.static_base, clean_path);
            match std::fs::read_to_string(&fqp) {
                Ok(_) => {
                    println!(
                        "[WARNING] Multiple Static renders are conflicting for path : {}",
                        fqd
                    )
                }
                _ => {}
            }
            match std::fs::create_dir_all(fqd) {
                Ok(_) => {}
                Err(e) => {
                    panic!("Fs error: {}", e)
                }
            }
            fqp
        }
        _ => {
            //file name is the link
            let link = content_store.name.as_ref().unwrap();
            let clean_path = link.trim().trim_end_matches("/").trim_start_matches("/");
            let fqd = format!("{}/{}", local_render_env.static_base, clean_path);
            match std::fs::create_dir_all(fqd) {
                Ok(_) => {}
                Err(e) => {
                    panic!("Fs error: {}", e)
                }
            }
            let fqp = format!("{}/{}/index.html", local_render_env.static_base, clean_path);
            fqp
        }
    }
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
            println!(
                "Reusing previous builds like cache not yet possible, rebuilding from scratch."
            );
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
        if path.is_file() {
            println!("rendering : {:?}", path);
            let content_store = MarkdownParse::parse(&path.display()).unwrap();
            let static_path =
                decide_static_serve_path(Rc::clone(&local_render_env), &content_store);
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
                    println!("\trendering to : {}", static_path);
                    std::fs::write(static_path, static_store).unwrap();
                }
                Err(e) => return Err(e),
            }
        }
    }
    //Copying css files over to static folder, we can serve these files through rocket.
    //This solution will NOT work with other ways to serving the site...Which is a future feature
    //to come [TODO]
    //We will anyways bundle css for now allowing users to use advanced css constructs
    match copy_css_files(Rc::clone(&local_render_env)) {
        Ok(_) => {}
        Err(e) => {
            return Err(CustomError {
                r#type: CustomErrorType::StaticRender,
                error: e.to_string(),
            })
        }
    }
    Ok(())
}

fn copy_css_files(local_render_env: Rc<RenderEnv>) -> Result<(), std::io::Error> {
    println!("Copying over CSS files : ");
    let content_walker = WalkDir::new(&local_render_env.css_base);
    for i in content_walker.into_iter() {
        let path = i.unwrap().into_path();
        if path.is_file() {
            let static_path = format!("{}/{}", local_render_env.static_base, path.display());
            println!("\t{}", static_path);
            let fs = FileProvider::new();
            let mut bundler = Bundler::new(&fs, None, ParserOptions::default());
            let stylesheet = bundler.bundle(Path::new(&path)).unwrap();
            let mut css_printer = PrinterOptions::default();
            css_printer.minify = true;
            let css_content = stylesheet.to_css(css_printer).unwrap().code;
            match std::fs::write(static_path, css_content) {
                Ok(_) => {}
                Err(e) => return Err(e),
            };
        } else {
            let static_path = format!("{}/{}", local_render_env.static_base, path.display());
            std::fs::create_dir(static_path).unwrap();
        }
    }
    Ok(())
}
