use std::collections::HashMap;

use rusticnotion::{
    models::{
        properties::PropertyValue, search::NotionSearch, Database, Page, PageCreateRequest,
        Properties,
    },
    NotionApi,
};

use crate::NewPage;

pub struct Notion {
    api: NotionApi,
}

impl Notion {
    pub fn new(api_token: String) -> Self {
        let api = NotionApi::new(api_token).unwrap();

        Notion { api }
    }

    pub async fn get_database_by_id(&self, database_id: String) -> Result<Database, String> {
        let search = NotionSearch::filter_by_databases();
        let response = self.api.search(search).await.unwrap().only_databases();

        if let Some(database) = response
            .results
            .iter()
            .find(|res| res.id.to_string().replace("-", "") == database_id)
        {
            if Notion::has_expected_database_properties(database.to_owned()) {
                Ok(database.to_owned())
            } else {
                Err(
                    "database does not have all required fields: Name, Image, URL, Tags"
                        .to_string(),
                )
            }
        } else {
            Err("database does not exist".to_string())
        }
    }

    fn has_expected_database_properties(database: Database) -> bool {
        let properties: Vec<&String> = database.properties.iter().map(|(name, _)| name).collect();
        const EXPECTED_DB_PROPERTIES: [&str; 4] = ["Name", "Image", "URL", "Tags"];

        // verify fields
        let db_has_expected_properties = EXPECTED_DB_PROPERTIES
            .iter()
            .map(|property| properties.contains(&&property.to_string()))
            .all(|x| x);

        db_has_expected_properties
    }

    pub async fn create_page(&self, new_page: NewPage) -> Result<Page, String> {
        let properties: Properties = Properties {
            properties: [
                ("Name".to_string(), new_page.get_name_property()),
                ("URL".to_string(), new_page.get_url_property()),
                ("Tags".to_string(), new_page.get_tags_property()),
                ("Image".to_string(), new_page.get_image_property()),
            ]
            .iter()
            .filter_map(|(key, property)| property.as_ref().map(|v| (key.clone(), v.clone())))
            .collect::<HashMap<String, PropertyValue>>(),
        };

        let page = PageCreateRequest {
            parent: rusticnotion::models::Parent::Database {
                database_id: new_page.parent_database.id,
            },
            properties,
            children: None, // TODO
        };

        let resp = self.api.create_page(page).await.unwrap();

        Ok(resp)
    }
}
