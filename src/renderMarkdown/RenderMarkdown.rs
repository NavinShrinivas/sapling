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

use crate::parseTemplate::ParseTemplate::TemplatesMetaData;
use crate::CustomErrorStage;
use crate::RenderEnv;
use crate::{loadMemory::LoadMemory, CustomError};
use futures::future::join_all;
use log::info;

//Renders all the markdown files with their templates, copies over css and asset files.
pub async fn parallel_static_render(
    local_render_env: &RenderEnv,
    template_meta: &TemplatesMetaData,
    full_content: &mut LoadMemory::Discovered,
) -> Result<(), CustomError> {
    let mut handles: Vec<_> = Vec::new();
    super::utils::clean_and_create_static(local_render_env).unwrap(); // Creates or clean the
                                                                      // static directory for build
    let parsed_templates: Vec<_> = template_meta
        .compiled_tera_instance
        .get_template_names()
        .collect();

    for (k, v) in full_content.data.iter_mut() {
        let path = k;
        let static_path = super::utils::decide_static_serve_path(
            &local_render_env,
            &v.frontmatter.as_ref().unwrap().get("link"),
            v.name.as_ref().unwrap(),
        );
        //need to inject the decided path into frontmatter (for use by rss later or something else)
        if let Some(f) = (*v).frontmatter.as_mut(){
            f["link"] = serde_yaml::Value::String(static_path.clone());
        } else {
            v.frontmatter = Some(serde_yaml::Value::Mapping(serde_yaml::Mapping::new()));
            v.frontmatter.as_mut().unwrap()["link"] = serde_yaml::Value::String(static_path.clone());
        }

        match super::utils::validate_template_request(
            &v.frontmatter.as_ref().unwrap().get("template"),
            local_render_env,
            &parsed_templates,
        ) {
            Ok(template_to_use) => {
                let job = tokio::spawn({
                    info!(
                        "rendering : {} , to : {}, template : {}",
                        path, static_path, template_to_use
                    );

                    let temp_context = match Context::from_serialize(&v) {
                        Ok(con) => con,
                        Err(e) => {
                            return Err(CustomError {
                                stage: CustomErrorStage::StaticRender,
                                error: format!(
                                    "[ERROR] Error parsing context from strcture in file {} : {}",
                                    path, e
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
                    parellel_final_render(static_store, static_path)
                });
                handles.push(job);
            }
            Err(e) => return Err(e),
        }
    }
    join_all(handles).await;
    //Copying css files over to static folder, we can serve these files through rocket.
    //This solution will NOT work with other ways to serving the site...Which is a future feature
    //to come [TODO]
    //We will anyways bundle css for now allowing users to use advanced css constructs
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

async fn parellel_final_render(
    static_store: String,
    static_path: String,
) -> Result<(), CustomError> {
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
            info!("processing : {}", static_path);
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
            info!("processing : {}", static_path);
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

pub fn static_render(
    local_render_env: &RenderEnv,
    template_meta: &TemplatesMetaData,
    full_content: &LoadMemory::Discovered,
) -> Result<(), CustomError> {
    super::utils::clean_and_create_static(local_render_env).unwrap();
    let parsed_templates: Vec<_> = template_meta
        .compiled_tera_instance
        .get_template_names()
        .collect();

    for (k, v) in full_content.data.iter() {
        let path = k;
        let content_store = v.clone();
        let static_path = super::utils::decide_static_serve_path(
            &local_render_env,
            &v.frontmatter.as_ref().unwrap().get("link"),
            v.name.as_ref().unwrap(),
        );
        match super::utils::validate_template_request(
            &v.frontmatter.as_ref().unwrap().get("template"),
            local_render_env,
            &parsed_templates,
        ) {
            Ok(template_to_use) => {
                info!(
                    "rendering : {} , to : {}, template : {}",
                    path, static_path, template_to_use
                );

                let temp_context = match Context::from_serialize(&content_store) {
                    Ok(con) => con,
                    Err(e) => {
                        return Err(CustomError {
                            stage: CustomErrorStage::StaticRender,
                            error: format!(
                                "[ERROR] Error parsing context from strcture in file {} : {}",
                                path, e
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
                final_render(static_store, static_path).unwrap();
            }
            Err(e) => return Err(e),
        }
    }
    //Copying css files over to static folder, we can serve these files through rocket.
    //This solution will NOT work with other ways to serving the site...Which is a future feature
    //to come [TODO]
    //We will anyways bundle css for now allowing users to use advanced css constructs
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
pub fn final_render(static_store: String, static_path: String) -> Result<(), CustomError> {
    log::info!("writing to : {}", static_path);
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
