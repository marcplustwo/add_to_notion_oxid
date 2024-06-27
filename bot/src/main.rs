#![allow(unused)]

use dotenvy::dotenv;
use notion::{NewPage, Notion};
use std::env;
use std::fmt::Error;

#[tokio::main]
async fn main() -> Result<(), String> {
    dotenv().expect(".env file not found");

    let notion_api_token = env::var("NOTION_API_TOKEN").expect("NOTION_API_TOKEN not set");
    let database_id = env::var("DATABASE_ID").expect("DATABASE_ID not set");

    let client = Notion::new(notion_api_token);
    let database = client.get_database_by_id(database_id).await.unwrap();

    let new_page = NewPage {
        parent_database: database,
        name: "new page".to_string(),
        url: Some("https://www.google.com/".to_string()),
        image_url: None,
        // TODO match existing tags, so they have the same color
        tags: Some(["beach".to_string(), "vacation".to_string()].to_vec()),
    };

    let page = client.create_page(new_page).await;

    Ok(())
}
