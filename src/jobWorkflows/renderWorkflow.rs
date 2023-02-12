use crate::loadMemory::LoadMemory;
use crate::parseTemplate::ParseTemplate;
use crate::renderMarkdown::RenderMarkdown;
use crate::renderMarkdown::ReverseIndex;
use log::{error, info };
pub fn renderJob(local_render_env: &'static crate::RenderEnv) -> Result<(), crate::CustomError> {
    let mut content_full_data = LoadMemory::Discovered::default();
    let template_meta = match ParseTemplate::TemplatesMetaData::new(&local_render_env) {
        Ok(s) => {
            info!("All detected templates parsed without errors!");
            s
        }
        Err(e) => {
            error!("Ran into error while parsing templates.");
            panic!("{}", e)
        }
    };
    let outer_rindex;
    match LoadMemory::discover_content(&local_render_env, &mut content_full_data) {
        Ok(reverseindex) => {
            info!("Detected all possible content (Markdown) file.");
            outer_rindex = reverseindex;
        }
        Err(e) => {
            panic!("{:?}", e)
        }
    }

    match RenderMarkdown::static_render(&local_render_env, &template_meta, &content_full_data) {
        Ok(_) => {
            info!("All markdown content rendered without errrors!")
        }
        Err(e) => {
            info!("Ran into error while rendering markdown.");
            panic!("{:?}", e)
        }
    }

    //[TODO] make this part of the renderMarkdown
    for (k, _) in outer_rindex.clone() {
        match ReverseIndex::reverse_index_render(
            k.to_string(),
            outer_rindex.clone(),
            &template_meta,
        ) {
            Ok(_) => {}
            Err(e) => {
                panic!("{:?}", e)
            }
        };
    }
    Ok(())
}
