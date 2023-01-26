use crate::CustomError;
use crate::CustomErrorStage;
use crate::TemplatesMetaData;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tera::Context;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ReverseRenderBody {
    reverseindexon : String,
    reverseindex: Vec<serde_yaml::value::Value>,
}

pub fn reverse_index_render(
    tag: String,
    reverseindex: HashMap<String, HashMap<String, Vec<serde_yaml::value::Value>>>,
    template_meta: &TemplatesMetaData,
) -> Result<(), CustomError> {
    println!("[INFO] rendering reverse indexes of {}",tag);
    for (k, v) in reverseindex.get(&tag).unwrap() {
        let local_serve_path = format!("static/{}/{}", tag, k.to_string());
        let local_serve_path_file = format!("{}/index.html", local_serve_path);
        println!("[INFO] rendering : {}", local_serve_path_file);
        std::fs::create_dir_all(&local_serve_path).unwrap();
        std::fs::File::create(&local_serve_path_file).unwrap();
        let template = format!("reverseindex/{}.html", tag);
        let temp_revser_body = ReverseRenderBody {
            reverseindexon : k.to_string(),
            reverseindex: v.to_vec(),
        };
        println!("{:#?}", temp_revser_body);
        final_reverse_render(
            template,
            &temp_revser_body,
            "test".to_string(),
            template_meta,
            local_serve_path_file,
        )
        .unwrap();
    }
    Ok(())
}

fn final_reverse_render(
    template_to_use: String,
    content_store: &ReverseRenderBody,
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
