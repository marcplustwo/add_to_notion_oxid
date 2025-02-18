use crate::db::Database;
use crate::handlers::command::{handle_command, Command};
use crate::handlers::dialogue::{
    instructions, receive_confirm, receive_database_id, receive_integration_token, State,
};
use crate::handlers::message::message_handler;
use crate::img_push::ImgPush;
use std::sync::Arc;
use teloxide::dispatching::dialogue::InMemStorage;
use teloxide::prelude::*;

pub async fn run_bot(db: Arc<Database>, img_push: Arc<ImgPush>) {
    let bot = Bot::from_env();

    let handler = Update::filter_message()
        .enter_dialogue::<Message, InMemStorage<State>, State>()
        .branch(
            dptree::entry()
                .filter_command::<Command>()
                .endpoint(handle_command),
        )
        .branch(
            dptree::filter(|msg: Message, db: Arc<Database>| {
                let user_details = db.get(&msg.chat.id.to_string()).unwrap();
                user_details.is_some()
            })
            .endpoint(message_handler),
        )
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
        );

    Dispatcher::builder(bot, handler)
        .dependencies(dptree::deps![InMemStorage::<State>::new(), db, img_push])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await
}
