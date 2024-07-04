use db::Database;
use dotenvy::dotenv;
use handle_dialogue::{
    instructions, receive_confirm, receive_database_id, receive_integration_token, State,
};
use handle_message::message_handler;
use img_push::ImgPush;
use std::env;
use std::sync::Arc;
use teloxide::dispatching::dialogue::InMemStorage;
use teloxide::prelude::*;

mod constants;
mod db;
mod handle_dialogue;
mod handle_message;
mod img_push;

#[tokio::main]
async fn main() -> Result<(), String> {
    println!("Starting the bot!");
    dotenv().expect(".env file not found");
    pretty_env_logger::init();

    let img_push_url = env::var("IMG_PUSH_URL").expect("IMG_PUSH_URL not set");

    let img_push = Arc::new(ImgPush::new(img_push_url));
    let db = Arc::new(Database::new("db.sqlite").unwrap());

    let bot = Bot::from_env();

    let handler = Update::filter_message()
        .branch(
            dptree::filter(|msg: Message, db: Arc<Database>| {
                let user_details = db.get(&msg.chat.id.to_string()).unwrap();
                user_details.is_some()
            })
            .endpoint(message_handler),
        )
        .enter_dialogue::<Message, InMemStorage<State>, State>()
        .branch(dptree::case![State::Instructions].endpoint(instructions))
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
        .branch(dptree::case![State::SetupComplete].endpoint(message_handler));

    Dispatcher::builder(bot, handler)
        .dependencies(dptree::deps![InMemStorage::<State>::new(), db, img_push])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;

    log::info!("Closing bot... Goodbye!");

    Ok(())
}
