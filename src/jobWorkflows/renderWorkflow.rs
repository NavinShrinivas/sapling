use crate::loadMemory::LoadMemory;
use crate::parseTemplate::ParseTemplate;
use crate::renderMarkdown::RenderMarkdown;
use crate::renderMarkdown::ReverseIndex;
use log::{error, info};
use crate::rss::rss;

pub async fn parallel_renderJob(
    local_render_env: &'static crate::RenderEnv,
    settings: &std::collections::HashMap<String, serde_yaml::value::Value>,
) -> Result<(), crate::CustomError> {
    //In this function, each step needs to complete before we can move forward.
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
    match LoadMemory::discover_content(&local_render_env, &mut content_full_data).await {
        Ok(reverseindex) => {
            info!("Detected all possible content (Markdown) file.");
            outer_rindex = reverseindex;
        }
        Err(e) => {
            panic!("{:?}", e)
        }
    }

    match RenderMarkdown::parallel_static_render(
        &local_render_env,
        &template_meta,
        &mut content_full_data,
    )
    .await
    {
        Ok(_) => {
            info!("All markdown content rendered without errors!")
        }
        Err(e) => {
            info!("Ran into error while rendering markdown.");
            panic!("{:?}", e)
        }
    }

    //[TODO] make this part of the renderMarkdown
    for (k, _) in outer_rindex.read().unwrap().clone() {
        match ReverseIndex::reverse_index_render(
            k.to_string(),
            outer_rindex.read().unwrap().to_owned(),
            &template_meta,
        ) {
            Ok(_) => {}
            Err(e) => {
                panic!("{:?}", e)
            }
        };
    }
    let render_rss = crate::settingYaml::settingYaml::get_inner_value(
        settings,
        vec!["rss".to_string(), "enable".to_string()],
        false,
    );
    if render_rss {
        info!("Rendering RSS feeds...");
        let rss_options = crate::settingYaml::settingYaml::get_inner_value(
            settings,
            vec!["rss".to_string()],
            rss::RssOptions::default(),
        );

        log::debug!("RSS options : {:?}", rss_options);
        match rss::generate_rss_map(
            &content_full_data, 
            &rss_options
        ) {
            map => {
                info!("All content rss items collected!");
                match rss::render_rss(
                    local_render_env,
                    &map,
                    &rss_options,
                ){
                    Ok(_) => {
                        info!("All RSS feeds rendered without errors!");
                    }
                    Err(e) => {
                        error!("Error rendering RSS feeds: {}", e);
                        panic!("{}", e)
                    }
                }
            }
        }
    } else {
        info!("RSS rendering is disabled.");
    }
    Ok(())
}
