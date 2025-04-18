use super::utils;
use crate::loadMemory::LoadMemory;
use crate::renderMarkdown;
use crate::settingYaml::settingYaml;
use chrono::Utc;
use rss_gen::{generate_rss, RssData, RssItem, RssVersion};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InternalRssItem {
    pub title: String,
    pub link: String,
    pub description: String,
    pub pub_date: String,
    pub build_date: String,
    pub author: String,
}

//Represents the RSS options in the config file
#[derive(Serialize, Deserialize, Debug)]
pub struct RssOptions {
    pub enable: bool,
    pub title: Option<String>,
    pub description: Option<String>,
    pub website_link: Option<String>,
    pub website_author: Option<String>,
    pub date_format: Option<String>,
    pub rss_groups: Vec<RssGroup>,
}

//For a given group, all these fields are compulsory:
#[derive(Serialize, Deserialize, Debug)]
pub struct RssGroup {
    pub name: String,
    pub group_id: String,
    pub link: String,
    pub content_url: String,
    pub description: String,
}

impl Default for RssOptions {
    fn default() -> Self {
        RssOptions {
            enable: false,
            title: None,
            description: None,
            website_link: None,
            website_author: None,
            date_format: Some("%d-%m-%Y".to_string()),
            rss_groups: vec![],
        }
    }
}

impl InternalRssItem {
    pub fn new(
        title: String,
        link: String,
        description: String,
        pub_date: String,
        build_date: String,
        author: String,
    ) -> Self {
        InternalRssItem {
            title,
            link,
            description,
            pub_date,
            build_date,
            author,
        }
    }
}

pub fn generate_rss_map(
    content_discover: &LoadMemory::Discovered,
    rss_options: &RssOptions,
) -> HashMap<String, Vec<InternalRssItem>> {
    let mut rss_map: HashMap<String, Vec<InternalRssItem>> = HashMap::new();
    let read_data_ref = &(content_discover.data);
    for (_, v) in read_data_ref.iter() {
        let f = &(v.frontmatter);
        let f = match f {
            Some(f) => f,
            None => continue, //If no frontmatter, skip
        };
        log::debug!("Processing rss gen for frontmatter : {:?}", f);
        match f["rss_group"].as_sequence() {
            Some(rss_groups) => {
                for rss_group in rss_groups {
                    let rss_key = rss_group.as_str().unwrap_or_default().to_string();
                    let inner_rss_item = InternalRssItem::new(
                        f["title"] //title is must, but if not default
                            .as_str()
                            .unwrap_or("A generic default title for posts")
                            .to_string(),
                        f["link"] //will always be present
                            .as_str()
                            .unwrap_or_default()
                            .to_string(),
                        f["description"] //may not be always present
                            .as_str()
                            .unwrap_or("A generic default description for posts")
                            .to_string(),
                        utils::convert_to_rfc822(
                            f["date"].as_str().unwrap_or(
                                //from content, or default now
                                Utc::now()
                                    .format(
                                        rss_options
                                            .date_format
                                            .as_ref()
                                            .unwrap_or(&"%d-%m-%Y".to_string()),
                                    )
                                    .to_string()
                                    .as_str(),
                            ),
                            rss_options
                                .date_format
                                .as_ref()
                                .unwrap_or(&"%d-%m-%Y".to_string())
                                .to_string(),
                        )
                        .to_string(),
                        utils::convert_to_rfc822(
                            //build date is always now
                            Utc::now()
                                .format(
                                    &rss_options
                                        .date_format
                                        .as_ref()
                                        .unwrap_or(&"%d-%m-%Y".to_string()),
                                )
                                .to_string()
                                .as_str(),
                            rss_options
                                .date_format
                                .as_ref()
                                .unwrap_or(&"%d-%m-%Y".to_string())
                                .to_string(),
                        ),
                        f["author"].as_str().unwrap_or("Jhon Doe").to_string(),
                    );
                    rss_map
                        .entry(rss_key)
                        .and_modify(|vec| vec.push(inner_rss_item.clone()))
                        .or_insert(vec![inner_rss_item.clone()]);
                }
            }
            None => match f["rss_group"].as_str() {
                Some(rss_group) => {
                    let rss_key = rss_group.to_string();
                    let inner_rss_item = InternalRssItem::new(
                        f["title"].as_str().unwrap_or_default().to_string(),
                        f["link"].as_str().unwrap_or_default().to_string(),
                        f["description"].as_str().unwrap_or_default().to_string(),
                        f["pub_date"].as_str().unwrap_or_default().to_string(),
                        f["build_date"].as_str().unwrap_or_default().to_string(),
                        f["author"].as_str().unwrap_or_default().to_string(),
                    );
                    rss_map
                        .entry(rss_key)
                        .and_modify(|vec| vec.push(inner_rss_item.clone()))
                        .or_insert(vec![inner_rss_item.clone()]);
                }
                None => {}
            },
        }
    }
    rss_map
}

pub fn render_rss(
    local_render_env: &crate::RenderEnv,
    rss_map: &HashMap<String, Vec<InternalRssItem>>,
    rss_options: &RssOptions,
) -> Result<(), String> {
    for (key, items) in rss_map.iter() {
        //if no content is for found a given group, it simply never executes that loop
        let rss_group = match rss_options
            .rss_groups
            .iter()
            .find(|group| group.group_id == *key)
        {
            Some(group) => group,
            None => {
                log::error!("Error finding rss group for key {}", key);
                continue;
            }
        };

        let mut rss_data: RssData = RssData::new(Some(RssVersion::RSS2_0))
            .title(key.to_string())
            .link(&rss_group.content_url)
            .description(&rss_group.description)
            .pub_date(Utc::now().format("%a, %d %b %Y %H:%M:%S GMT").to_string())
            .last_build_date(Utc::now().format("%a, %d %b %Y %H:%M:%S GMT").to_string())
            .author(
                rss_options
                    .website_author
                    .as_ref()
                    .unwrap_or(&"Jhon Doe".to_string())
                    .to_string(),
            );

        for item in items.iter() {
            let rss_item = RssItem::new()
                .title(item.title.clone())
                .link(format!("{}{}", rss_group.content_url, item.link.clone()))
                .description(item.description.clone())
                .pub_date(item.pub_date.clone());
            rss_data.add_item(rss_item);
        }
        let rss_data_string = match generate_rss(&rss_data) {
            Ok(rss_data_string) => rss_data_string,
            Err(e) => {
                log::error!(
                    "Error generating rss for group {} : {} {:?}",
                    key,
                    e,
                    rss_data
                );
                continue;
            }
        };
        let static_path = utils::decide_static_rss_render_path(local_render_env, &rss_group.link);
        match renderMarkdown::RenderMarkdown::final_render(rss_data_string, static_path) {
            Ok(_) => {
                log::info!("Successfully rendered rss for group {}", key);
            }
            Err(e) => {
                log::error!("Error rendering rss for group {} : {}", key, e.error);
                continue;
            }
        }
    }
    Ok(())
}
