#![allow(unused)]

use dotenvy::dotenv;
use handle_dialogue::{
    receive_confirm, receive_database_id, receive_integration_token, start, State,
};
use handle_message::message_handler;
use img_push::ImgPush;
use notion::{NewPage, Notion};
use std::env;
use std::fmt::Error;
use std::sync::Arc;
use teloxide::dispatching::dialogue::InMemStorage;
use teloxide::prelude::*;

mod handle_dialogue;
mod handle_message;
mod img_push;

#[tokio::main]
async fn main() -> Result<(), String> {
    dotenv().expect(".env file not found");
    pretty_env_logger::init();

    let notion_api_token = env::var("NOTION_API_TOKEN").expect("NOTION_API_TOKEN not set");
    let img_push_url = env::var("IMG_PUSH_URL").expect("IMG_PUSH_URL not set");

    let notion = Arc::new(Notion::new(notion_api_token));
    let img_push = Arc::new(ImgPush::new(img_push_url));

    let bot = Bot::from_env();

    let handler = Update::filter_message()
        .enter_dialogue::<Message, InMemStorage<State>, State>()
        .branch(dptree::case![State::Start].endpoint(start))
        .branch(dptree::case![State::ReceiveIntegrationToken].endpoint(receive_integration_token))
        .branch(
            dptree::case![State::ReceiveDatabaseId { integration_token }]
                .endpoint(receive_database_id),
        )
        .branch(
            dptree::case![State::Confirm {
                integration_token,
                database_id
            }]
            .endpoint(receive_confirm),
        )
        .branch(Update::filter_message().endpoint(message_handler));

    Dispatcher::builder(bot, handler)
        .dependencies(dptree::deps![
            InMemStorage::<State>::new(),
            notion,
            img_push
        ])
        // .dependencies(dptree::deps![notion, img_push])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;

    // let handler = dptree::entry().branch(Update::filter_message().endpoint(message_handler));

    // Dispatcher::builder(bot, handler)
    //     .dependencies(dptree::deps![notion, img_push])
    //     .enable_ctrlc_handler()
    //     .build()
    //     .dispatch()
    //     .await;

    log::info!("Closing bot... Goodbye!");

    // dialogue: get notion_api_key and database?

    Ok(())
}
