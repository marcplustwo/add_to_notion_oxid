use std::process::exit;

use teloxide::{dispatching::dialogue::InMemStorage, prelude::*};

type SetupDialogue = Dialogue<State, InMemStorage<State>>;
type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

#[derive(Clone, Default)]
pub enum State {
    #[default]
    Start,
    ReceiveIntegrationToken,
    ReceiveDatabaseId {
        integration_token: String,
    },
    Confirm {
        integration_token: String,
        database_id: String,
    },
}

pub async fn start(bot: Bot, dialogue: SetupDialogue, msg: Message) -> HandlerResult {
    // TODO: instructions message
    bot.send_message(msg.chat.id, "This will be the instructions message")
        .await?;
    bot.send_message(msg.chat.id, "Send the integration_token")
        .await?;
    dialogue.update(State::ReceiveIntegrationToken).await?;
    Ok(())
}

pub async fn receive_integration_token(
    bot: Bot,
    dialogue: SetupDialogue,
    msg: Message,
) -> HandlerResult {
    match msg.text() {
        Some(text) => {
            bot.send_message(msg.chat.id, "Send the database_id")
                .await?;
            dialogue
                .update(State::ReceiveDatabaseId {
                    integration_token: text.to_owned(),
                })
                .await?;
        }
        None => {
            bot.send_message(msg.chat.id, "Send me plain text.").await?;
        }
    }

    Ok(())
}

pub async fn receive_database_id(
    bot: Bot,
    dialogue: SetupDialogue,
    msg: Message,
    integration_token: String,
    database_id: String,
) -> HandlerResult {
    match msg.text() {
        Some(text) => {
            let report =
                format!("integration_token: {integration_token}, database_id {database_id}");
            bot.send_message(msg.chat.id, report).await?;
            bot.send_message(msg.chat.id, "Please confirm").await?;
            dialogue
                .update(State::Confirm {
                    integration_token,
                    database_id: text.to_owned(),
                })
                .await?;
        }
        None => {
            bot.send_message(msg.chat.id, "Send me plain text.").await?;
        }
    }

    Ok(())
}

pub async fn receive_confirm(bot: Bot, dialogue: SetupDialogue, msg: Message) -> HandlerResult {
    match msg.text() {
        Some(text) => {
            bot.send_message(msg.chat.id, "Ok!").await?;
            dialogue.exit().await?;
        }
        None => {
            bot.send_message(msg.chat.id, "Send me plain text.").await?;
        }
    }

    Ok(())
}

// async fn receive_age(
//     bot: Bot,
//     dialogue: SetupDialogue,
//     full_name: String, // Available from `State::ReceiveAge`.
//     msg: Message,
// ) -> HandlerResult {
//     match msg.text().map(|text| text.parse::<u8>()) {
//         Some(Ok(age)) => {
//             bot.send_message(msg.chat.id, "What's your location?").await?;
//             dialogue.update(State::ReceiveLocation { full_name, age }).await?;
//         }
//         _ => {
//             bot.send_message(msg.chat.id, "Send me a number.").await?;
//         }
//     }

//     Ok(())
// }

// async fn receive_location(
//     bot: Bot,
//     dialogue: MyDialogue,
//     (full_name, age): (String, u8), // Available from `State::ReceiveLocation`.
//     msg: Message,
// ) -> HandlerResult {
//     match msg.text() {
//         Some(location) => {
//             let report = format!("Full name: {full_name}\nAge: {age}\nLocation: {location}");
//             bot.send_message(msg.chat.id, report).await?;
//             dialogue.exit().await?;
//         }
//         None => {
//             bot.send_message(msg.chat.id, "Send me plain text.").await?;
//         }
//     }

//     Ok(())
// }
