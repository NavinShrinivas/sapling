use crate::{
    CustomError, CustomErrorStage, RenderEnv,
};
use std::fs::DirBuilder;
use std::os::unix::fs::DirBuilderExt;

use log::{info, warn};
use serde_yaml::Value;


pub fn validate_template_request(
    requested_frontmatter_template: &Option<&Value>,
    local_render_env: &RenderEnv,
    parsed_templates: &Vec<&str>
) -> Result<String, CustomError> {
    let requested_template = match requested_frontmatter_template {
        Some(template_path) => template_path.as_str().unwrap(),
        _ => {
            warn!("No templates property found in frontmatter, defaulting.");
            &local_render_env.default_template
        }
    };
    if parsed_templates.contains(&requested_template) {
        return Ok(requested_template.to_string());
    } else {
        return Err(CustomError {
            stage: CustomErrorStage::StaticRender,
            error: String::from("Error finding a proper template to render markdown."),
        });
    }
}


pub fn decide_static_serve_path(
    local_render_env: &RenderEnv,
    requested_frontmatter_serve_path : &Option<&Value>,
    file_name :  &str
) -> String {
    //First check if the frontmatter has `link`
    //
    match requested_frontmatter_serve_path{
        //unwrap is fine, we are sure no None
        Some(link) => {
            let link_str = match link.as_str(){
                Some(v) => v.to_string(), 
                None => panic!("You seemed to have given an invalid serve path for some markdown content.")
            };
            let clean_path = link_str
                .trim()
                .trim_end_matches("/")
                .trim_start_matches("/");
            let fqd = format!("{}/{}", local_render_env.static_base, clean_path);
            let fqp = format!("{}/{}/index.html", local_render_env.static_base, clean_path);
            match std::fs::read_to_string(&fqp) {
                Ok(_) => {
                    warn!("Multiple Static renders are conflicting for path : {}", fqd)
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
            warn!("Link tag not found in frontmatter,using name.");
            let link = file_name; //unwrap is fine, we are sure about no None
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

pub fn clean_and_create_static(local_render_env: &RenderEnv) -> Result<(), CustomError> {
    match std::fs::remove_dir_all(&local_render_env.static_base) {
        Ok(_) => {
            info!("Reusing previous builds like cache not yet possible, rebuilding from scratch.");
            //[TODO] use cached static sites
        }
        _ => {
            info!("Static dir not found, building from scratch! Reusing builds not yet possible.");
        }
    }
    let mut builder = DirBuilder::new();
    builder.mode(0o755);
    match builder.create(&local_render_env.static_base) {
        Ok(_) => Ok(()),
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
}
