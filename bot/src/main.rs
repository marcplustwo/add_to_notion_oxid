#![allow(unused)]

use dotenvy::dotenv;
use handlers::message_handler;
use notion::{NewPage, Notion};
use std::env;
use std::fmt::Error;
use std::sync::Arc;

use teloxide::prelude::*;

mod handlers;
mod image_upload;

#[tokio::main]
async fn main() -> Result<(), String> {
    dotenv().expect(".env file not found");

    let notion_api_token = env::var("NOTION_API_TOKEN").expect("NOTION_API_TOKEN not set");

    let notion = Arc::new(Notion::new(notion_api_token));

    let handler = dptree::entry().branch(Update::filter_message().endpoint(message_handler));

    let bot = Bot::from_env();
    Dispatcher::builder(bot, handler)
        .dependencies(dptree::deps![notion])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;

    log::info!("Closing bot... Goodbye!");

    // dialogue: get notion_api_key and database?

    Ok(())
}
