use std::fs::DirBuilder;
use std::os::unix::fs::DirBuilderExt;
use crate::{parseTemplate::ParseTemplate::TemplatesMetaData, CustomError, RenderEnv, CustomErrorStage};

use log::{info,warn };

pub fn validate_template_request(
    frontmatter: &serde_yaml::Value,
    local_render_env: &RenderEnv,
    template_meta: &TemplatesMetaData,
) -> Result<String, CustomError> {
    let requested_template = match frontmatter.get("template") {
        Some(template_path) => template_path.as_str().unwrap(),
        _ => {
            warn!("No templates property found in frontmatter, defaulting.");
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

pub fn decide_static_serve_path(
    local_render_env: &RenderEnv,
    content_store: &crate::parseMarkdown::ParseMarkdown::ContentDocument,
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
                    warn!(
                        "Multiple Static renders are conflicting for path : {}",
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
            warn!("Link tag not found in frontmatter,using name.");
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

pub fn clean_and_create_static(local_render_env: &RenderEnv) -> Result<(), CustomError>{
        match std::fs::remove_dir_all(&local_render_env.static_base) {
        Ok(_) => {
            info!(
                "Reusing previous builds like cache not yet possible, rebuilding from scratch."
            );
            //[TODO] use cached static sites
        }
        _ => {
            info!(
                "Static dir not found, building from scratch! Reusing builds not yet possible."
            );
        }
    }
    let mut builder = DirBuilder::new();
    builder.mode(0o755);
    match builder.create(&local_render_env.static_base) {
        Ok(_) => {Ok(())}
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
