#[allow(dead_code)]
#[allow(non_snake_case)]
use lightningcss::{
    bundler::{Bundler, FileProvider},
    printer::PrinterOptions,
    stylesheet::ParserOptions,
};
use std::path::Path;
use std::rc::Rc;
use tera::Context;
use walkdir::WalkDir;

use crate::markdown_parser::MarkdownParse;
use crate::CustomError;
use crate::CustomErrorStage;
use crate::RenderEnv;
use crate::TemplatesMetaData;

fn decide_static_serve_path(
    local_render_env: Rc<RenderEnv>,
    content_store: &crate::markdown_parser::MarkdownParse::ContentDocument,
) -> String {
    //First check if the frontmatter has `link`
    match &content_store.frontmatter.as_ref().unwrap().get("link") {
        //unwrap is fine, we are sure no None
        Some(link) => {
            let link_str = link.as_str().unwrap().to_string(); //unwrap is fine
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
            println!("\t[INFO|WARN] Link tag not found in frontmatter,using name.");
            let link = &content_store.name.as_ref().unwrap(); //unwrap is fine, we are sure about no None
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
            println!("\t[INFO|WARN] No templates property found in frontmatter, defaulting.");
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
            stage: CustomErrorStage::StaticRender,
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
    match std::fs::create_dir(&local_render_env.static_base) {
        Ok(_) => {}
        Err(e) => {
            return Err(CustomError {
                stage: CustomErrorStage::StaticRender,
                error: format!(
                "[ERROR] Failed creating static output dir, possibly not enough permission. : {}",
                e
            ),
            })
        }
    }
    let content_walker = WalkDir::new(&local_render_env.content_base);
    for i in content_walker.into_iter() {
        let entry = match i {
            Ok(entry) => {entry},
            Err(e) => {
                return Err(CustomError {
                    stage: CustomErrorStage::StaticRender,
                    error: format!("[ERROR] Dir entry error : {}", e),
                })
            }
        };
        let path = entry.path();
        if path.is_file() {
            println!("[INFO] Rendering : {:?}", path);
            let content_store =
                match MarkdownParse::parse(&path.display()) {
                    Ok(content) => content.unwrap(), //unwrap is fine
                    Err(e) => return Err(e),
                };
            let static_path =
                decide_static_serve_path(Rc::clone(&local_render_env), &content_store);
            match validate_template_request(
                &content_store.frontmatter.as_ref().unwrap(),
                Rc::clone(&local_render_env),
                &template_meta,
            ) {
                Ok(template_to_use) => {
                    println!("\ttemplate : {}", template_to_use);
                    let temp_context = match Context::from_serialize(&content_store) {
                        Ok(con) => con,
                        Err(e) => {
                            return Err(CustomError {
                                stage: CustomErrorStage::StaticRender,
                                error: format!(
                                    "[ERROR] Error parsing context from strcture in file {} : {}",
                                    path.display(),
                                    e
                                ),
                            })
                        }
                    };
                    let static_store = match template_meta
                        .compiled_tera_instance
                        .render(&template_to_use, &temp_context)
                    {
                        Ok(stat) => stat,
                        Err(e) => {
                            return Err(CustomError {
                                stage: CustomErrorStage::StaticRender,
                                error: format!("[ERROR] Error rendering static files : {}", e),
                            })
                        }
                    };
                    println!("\trendering to : {}", static_path);
                    match std::fs::write(static_path, static_store){
                        Ok(_) =>{},
                        Err(e) => {
                            return Err(CustomError{
                                stage : CustomErrorStage::StaticRender,
                                error : format!("[ERROR] Error writing static files to respective paths : {}",e)
                            })
                        }
                    }
                }
                Err(e) => return Err(e),
            }
        }
    }
    //Copying css files over to static folder, we can serve these files through rocket.
    //This solution will NOT work with other ways to serving the site...Which is a future feature
    //to come [TODO]
    //We will anyways bundle css for now allowing users to use advanced css constructs
    println!("[INFO] bundling and copying over CSS files to static paths.");
    match copy_css_files(Rc::clone(&local_render_env)) {
        Ok(_) => {}
        Err(e) => {
            return Err(e)
        }
    }
    Ok(())
}

fn copy_css_files(local_render_env: Rc<RenderEnv>) -> Result<(), CustomError> {
    println!("Copying over CSS files : ");
    let content_walker = WalkDir::new(&local_render_env.css_base);
    for i in content_walker.into_iter() {
        let entry = match i {
            Ok(entry) => {entry},
            Err(e) => {
                return Err(CustomError {
                    stage: CustomErrorStage::StaticRender,
                    error: format!("[ERROR] Dir entry error : {}", e),
                })
            }
        };
        let path = entry.path();
        if path.is_file() {
            let static_path = format!("{}/{}", local_render_env.static_base, path.display());
            println!("\tprocessing : {}", static_path);
            let fs = FileProvider::new();
            let mut bundler = Bundler::new(&fs, None, ParserOptions::default());
            let stylesheet = match bundler.bundle(Path::new(&path)){
                Ok(stylesheet) => {stylesheet},
                Err(e) => {
                    return Err(CustomError{
                        stage : CustomErrorStage::StaticRender,
                        error : format!("[ERROR] Error creating a bundler : {}",e)
                    })
                }
            };
            let mut css_printer = PrinterOptions::default();
            css_printer.minify = true;
            let css_content = match stylesheet.to_css(css_printer){
                Ok(s) => {s.code},
                Err(e) =>{
                    return Err(CustomError{
                        stage : CustomErrorStage::StaticRender,
                        error : format!("[ERROR] Error bundling css and rendering it : {}",e)
                    })
                }
            };
            match std::fs::write(static_path, css_content) {
                Ok(_) => {}
                Err(e) => return Err(CustomError{
                    stage : CustomErrorStage::StaticRender,
                    error : format!("[ERROR] error writing css files to static dir : {}",e)
                })
            };
        } else {
            let static_path = format!("{}/{}", local_render_env.static_base, path.display());
            match std::fs::create_dir(static_path){
                Ok(_) => {},
                Err(e) =>{
                    return Err(CustomError{
                        stage : CustomErrorStage::StaticRender,
                        error : format!("[ERROR] Error creating directory for css files : {}",e)
                    })
                }

            };
        }
    }
    Ok(())
}
