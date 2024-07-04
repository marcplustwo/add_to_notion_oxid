use std::{error::Error, sync::Arc};

use teloxide::{prelude::*, utils::command::BotCommands};

use crate::db::Database;

use super::dialogue::{SetupDialogue, State};

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
pub enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "resets the bot and deletes the user's tokens")]
    Reset,
}

pub async fn handle_command(
    bot: Bot,
    msg: Message,
    cmd: Command,
    dialogue: SetupDialogue,
    db: Arc<Database>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    match cmd {
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string())
                .await?;
        }
        Command::Reset => {
            db.delete(&msg.chat.id.to_string())?;
            bot.send_message(
                msg.chat.id,
                format!("Try again by sending a message to activate the setup"),
            )
            .await?;
            dialogue.update(State::Instructions).await?;
        }
    };

    Ok(())
}
