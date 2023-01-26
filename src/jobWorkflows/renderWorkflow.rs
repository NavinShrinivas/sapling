use crate::loadMemory::LoadMemory;
use crate::parseTemplate::ParseTemplate;
use crate::renderMarkdown::RenderMarkdown;
use crate::renderMarkdown::ReverseIndex;

pub fn renderJob(local_render_env: &crate::RenderEnv) -> Result<(), crate::CustomError> {
    let mut content_full_data = crate::Discovered::default();
    let template_meta = match ParseTemplate::TemplatesMetaData::new(&local_render_env) {
        Ok(s) => {
            println!("All detected templates parsed without errors!");
            s
        }
        Err(e) => {
            println!("Ran into error while parsing templates.");
            panic!("{}", e)
        }
    };
    let outer_rindex;
    match LoadMemory::discover_content(&local_render_env, &mut content_full_data) {
        Ok(reverseindex) => {
            println!("Detected all possible content (Markdown) file.");
            outer_rindex = reverseindex;
        }
        Err(e) => {
            panic!("{:?}", e)
        }
    }

    match RenderMarkdown::static_render(&local_render_env, &template_meta, &content_full_data) {
        Ok(_) => {
            println!("All markdown content rendered without errrors!")
        }
        Err(e) => {
            println!("Ran into error while rendering markdown.");
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
