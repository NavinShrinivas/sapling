use crate::{markdown_parser::MarkdownParse, CustomError, CustomErrorStage, Discovered, RenderEnv};
use walkdir::WalkDir;

pub fn discover_content(
    local_render_env: &RenderEnv,
    content_full_data: &mut Discovered,
) -> Result<(), CustomError> {
    let content_walker = WalkDir::new(&local_render_env.content_base);
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
            println!("[INFO] Detected : {:?}", path);
            let content_store = match MarkdownParse::parse(&path.display()) {
                Ok(content) => content.unwrap(), //unwrap is fine
                Err(e) => return Err(e),
            };
            content_full_data
                .data
                .insert(path.display().to_string(), content_store);
        }
    }
    Ok(())
}
