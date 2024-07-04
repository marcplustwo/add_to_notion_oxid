use std::sync::Arc;

use teloxide::{dispatching::dialogue::InMemStorage, prelude::*, types::ParseMode};

use crate::{
    constants::INSTRUCTIONS_MSG,
    db::{Database, UserDetails},
};

pub type SetupDialogue = Dialogue<State, InMemStorage<State>>;
type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

#[derive(Clone, Default)]
pub enum State {
    #[default]
    Instructions,
    ReceiveIntegrationToken,
    ReceiveDatabaseId {
        integration_token: String,
    },
    Confirm {
        integration_token: String,
        database_id: String,
    },
    SetupComplete,
}

pub async fn instructions(bot: Bot, dialogue: SetupDialogue, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id, INSTRUCTIONS_MSG)
        .parse_mode(ParseMode::MarkdownV2)
        .await?;
    bot.send_message(msg.chat.id, "Please pass me the Notion integration token")
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
            bot.send_message(msg.chat.id, "Please send the database id now")
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
) -> HandlerResult {
    match msg.text() {
        Some(text) => {
            let database_id = text.to_owned();

            let report =
                format!("Integration Token: {integration_token}\nDatabase ID: {database_id}");

            bot.send_message(msg.chat.id, "Please confirm the following data with _yes_")
                .parse_mode(ParseMode::MarkdownV2)
                .await?;
            bot.send_message(msg.chat.id, report).await?;

            dialogue
                .update(State::Confirm {
                    integration_token,
                    database_id,
                })
                .await?;
        }
        None => {
            bot.send_message(msg.chat.id, "Send me plain text.").await?;
        }
    }

    Ok(())
}

pub async fn receive_confirm(
    bot: Bot,
    dialogue: SetupDialogue,
    msg: Message,
    db: Arc<Database>,
    (integration_token, database_id): (String, String),
) -> HandlerResult {
    match msg.text() {
        Some(text) => {
            if text.to_lowercase().contains("yes") {
                bot.send_message(
                    msg.chat.id,
                    "You can now start feeding your Notion Webdump!",
                )
                .await?;

                db.register(UserDetails {
                    user_id: msg.chat.id.to_string(),
                    integration_token,
                    database_id,
                })?;

                // dialogue.exit().await?;
                dialogue.update(State::SetupComplete).await?;
            } else {
                bot.send_message(msg.chat.id, format!("Try again")).await?;
                dialogue.update(State::Instructions).await?;
            }
        }
        None => {
            bot.send_message(msg.chat.id, "Send me plain text.").await?;
        }
    }

    Ok(())
}
