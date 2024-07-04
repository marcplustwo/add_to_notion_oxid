// use std::{env, sync::Arc};
// use notion::Notion;
// use teloxide::prelude::*;

// use crate::handlers::message_handler;

// pub async fn run() {
//     let bot = Bot::from_env();


//     let notion = Arc::new(Notion::new());

//     let handler = dptree::entry()
//         .branch(Update::filter_message().endpoint(message_handler));
//         // .branch(Update::filter_callback_query().endpoint(callback_handler));

//     Dispatcher::builder(bot, handler)
//         .dependencies(dptree::deps![notion])
//         .enable_ctrlc_handler()
//         .build()
//         .dispatch()
//         .await;

//     log::info!("Closing bot... Goodbye!");
// }