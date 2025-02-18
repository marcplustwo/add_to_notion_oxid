use super::NewPage;
use anyhow::{anyhow, Result};
use rusticnotion::{
    models::{
        block::{BookmarkFields, CreateBlock, ExternalFileObject},
        properties::{PropertyConfiguration, PropertyValue},
        search::NotionSearch,
        Database, Page, PageCreateRequest, Properties,
    },
    NotionApi,
};
use std::collections::HashMap;

pub struct Notion {
    pub api: NotionApi,
}

impl Notion {
    pub fn has_expected_database_properties(database: &Database) -> bool {
        const EXPECTED_DB_PROPERTIES: [&str; 4] = ["Name", "Image", "URL", "Tags"];

        // verify fields
        let db_has_expected_properties = EXPECTED_DB_PROPERTIES
            .iter()
            .map(|property| database.properties.contains_key(&property.to_string()))
            .all(|x| x);

        db_has_expected_properties
    }

    pub fn new(api_token: String) -> Self {
        let api = NotionApi::new(api_token).unwrap();

        Notion { api }
    }

    pub async fn get_database_by_id(&self, database_id: String) -> Result<Database> {
        let search = NotionSearch::filter_by_databases();
        let response = self.api.search(search).await?.only_databases();

        if let Some(database) = response
            .results
            .iter()
            .find(|res| res.id.to_string().replace("-", "") == database_id)
        {
            Ok(database.to_owned())
        } else {
            Err(anyhow!("database does not exist"))
        }
    }

    pub async fn create_page(&self, new_page: NewPage) -> Result<Page, String> {
        let existing_tags = match new_page.database.properties.get("Tags").unwrap() {
            PropertyConfiguration::MultiSelect {
                id: _,
                multi_select,
            } => multi_select.options.clone(),
            _ => vec![],
        };

        let properties: Properties = Properties {
            properties: [
                ("Name".to_string(), new_page.get_name_property()),
                ("URL".to_string(), new_page.get_url_property()),
                (
                    "Tags".to_string(),
                    new_page.get_tags_property(existing_tags),
                ),
                ("Image".to_string(), new_page.get_image_property()),
            ]
            .iter()
            .filter_map(|(key, property)| property.as_ref().map(|v| (key.clone(), v.clone())))
            .collect::<HashMap<String, PropertyValue>>(),
        };

        let image_block: Option<CreateBlock> = if let Some(image_url) = new_page.image_url {
            Some(CreateBlock::Image {
                image: rusticnotion::models::block::FileObject::External {
                    external: ExternalFileObject { url: image_url },
                },
            })
        } else {
            None
        };

        let bookmark: Option<CreateBlock> = if let Some(url) = new_page.url {
            Some(CreateBlock::Bookmark {
                bookmark: BookmarkFields {
                    url,
                    caption: vec![],
                },
            })
        } else {
            None
        };

        let blocks: Option<Vec<Option<CreateBlock>>> = Some([image_block, bookmark].to_vec());

        let children = if let Some(blocks) = blocks {
            let children = blocks
                .iter()
                .filter(|block| block.is_some())
                .map(|block| block.to_owned().unwrap())
                .collect::<Vec<CreateBlock>>();
            Some(children)
        } else {
            None
        };

        let page = PageCreateRequest {
            parent: rusticnotion::models::Parent::Database {
                database_id: new_page.database.id,
            },
            properties,
            children,
        };

        let resp = self.api.create_page(page).await.unwrap();

        Ok(resp)
    }
}
