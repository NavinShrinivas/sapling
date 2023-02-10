#[allow(dead_code)]
#[allow(non_snake_case)]
use lightningcss::{
    bundler::{Bundler, FileProvider},
    printer::PrinterOptions,
    stylesheet::ParserOptions,
};
use std::path::Path;
use tera::Context;
use walkdir::WalkDir;

use crate::{CustomError, loadMemory::LoadMemory};
use crate::CustomErrorStage;
use crate::RenderEnv;
use crate::parseTemplate::ParseTemplate::TemplatesMetaData;

pub fn static_render(
    local_render_env: &RenderEnv,
    template_meta: &TemplatesMetaData,
    full_content: &LoadMemory::Discovered,
) -> Result<(), CustomError> {
    super::utils::clean_and_create_static(local_render_env).unwrap();
    for (k, v) in full_content.data.iter() {
        let path = k;
        println!("[INFO] Rendering : {:?}", path);
        let content_store = v;
        let static_path = super::utils::decide_static_serve_path(&local_render_env, &content_store);
        match super::utils::validate_template_request(
            &content_store.frontmatter.as_ref().unwrap(),
            local_render_env,
            &template_meta,
        ) {
            Ok(template_to_use) => {
                final_render(template_to_use, &content_store,path.to_string(),template_meta,static_path).unwrap();
            }
            Err(e) => return Err(e),
        }
    }
    //Copying css files over to static folder, we can serve these files through rocket.
    //This solution will NOT work with other ways to serving the site...Which is a future feature
    //to come [TODO]
    //We will anyways bundle css for now allowing users to use advanced css constructs
    println!("[INFO] bundling and copying over CSS files to static paths.");
    match copy_css_files(local_render_env) {
        Ok(_) => {}
        Err(e) => return Err(e),
    }
    match copy_assets_files(local_render_env) {
        Ok(_) => {}
        Err(e) => return Err(e),
    }

    Ok(())
}

fn final_render(
    template_to_use: String,
    content_store: &crate::parseMarkdown::ParseMarkdown::ContentDocument,
    path: String,
    template_meta: &TemplatesMetaData,
    static_path: String,
) -> Result<(), CustomError> {
    println!("\ttemplate : {}", template_to_use);
    let temp_context = match Context::from_serialize(&content_store) {
        Ok(con) => con,
        Err(e) => {
            return Err(CustomError {
                stage: CustomErrorStage::StaticRender,
                error: format!(
                    "[ERROR] Error parsing context from strcture in file {} : {}",
                    path,
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
    match std::fs::write(static_path, static_store) {
        Ok(_) => return Ok(()),
        Err(e) => {
            return Err(CustomError {
                stage: CustomErrorStage::StaticRender,
                error: format!(
                    "[ERROR] Error writing static files to respective paths : {}",
                    e
                ),
            })
        }
    }
}

fn copy_css_files(local_render_env: &RenderEnv) -> Result<(), CustomError> {
    println!("Copying over CSS files : ");
    let content_walker = WalkDir::new(&local_render_env.css_base);
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
            let static_path = format!("{}/{}", local_render_env.static_base, path.display());
            println!("\tprocessing : {}", static_path);
            let fs = FileProvider::new();
            let mut bundler = Bundler::new(&fs, None, ParserOptions::default());
            let stylesheet = match bundler.bundle(Path::new(&path)) {
                Ok(stylesheet) => stylesheet,
                Err(e) => {
                    return Err(CustomError {
                        stage: CustomErrorStage::StaticRender,
                        error: format!("[ERROR] Error creating a bundler : {}", e),
                    })
                }
            };
            let mut css_printer = PrinterOptions::default();
            css_printer.minify = true;
            let css_content = match stylesheet.to_css(css_printer) {
                Ok(s) => s.code,
                Err(e) => {
                    return Err(CustomError {
                        stage: CustomErrorStage::StaticRender,
                        error: format!("[ERROR] Error bundling css and rendering it : {}", e),
                    })
                }
            };
            match std::fs::write(static_path, css_content) {
                Ok(_) => {}
                Err(e) => {
                    return Err(CustomError {
                        stage: CustomErrorStage::StaticRender,
                        error: format!("[ERROR] error writing css files to static dir : {}", e),
                    })
                }
            };
        } else {
            let static_path = format!("{}/{}", local_render_env.static_base, path.display());
            match std::fs::create_dir(static_path) {
                Ok(_) => {}
                Err(e) => {
                    return Err(CustomError {
                        stage: CustomErrorStage::StaticRender,
                        error: format!("[ERROR] Error creating directory for css files : {}", e),
                    })
                }
            };
        }
    }
    Ok(())
}

fn copy_assets_files(local_render_env: &RenderEnv) -> Result<(), CustomError> {
    println!("Copying over asset files : ");
    let content_walker = WalkDir::new(&local_render_env.assets_base);
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
            let static_path = format!("{}/{}", local_render_env.static_base, path.display());
            println!("\tprocessing : {}", static_path);
            std::fs::write(&static_path, "").unwrap();
            match std::fs::copy(path, static_path) {
                Ok(_) => {}
                Err(e) => {
                    return Err(CustomError {
                        stage: CustomErrorStage::StaticRender,
                        error: format!("[ERROR] error writing assets files to static dir : {}", e),
                    })
                }
            };
        } else {
            let static_path = format!("{}/{}", local_render_env.static_base, path.display());
            match std::fs::create_dir(static_path) {
                Ok(_) => {}
                Err(e) => {
                    return Err(CustomError {
                        stage: CustomErrorStage::StaticRender,
                        error: format!("[ERROR] Error creating directory for assets files : {}", e),
                    })
                }
            };
        }
    }
    Ok(())
}
