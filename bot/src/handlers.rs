use std::env;
use std::{error::Error, sync::Arc};

use notion::{NewPage, Notion};
use regex::Regex;
use teloxide::net::Download;
use teloxide::payloads::{EditMessageTextSetters, GetFile};
use teloxide::types::{ParseMode, PhotoSize};
use teloxide::utils::html::link;
use teloxide::{prelude::*, utils::command::BotCommands};
use tokio::fs;

use crate::img_push::ImgPush;

// #[derive(BotCommands, Clone)]
// #[command(
//     rename_rule = "lowercase",
//     description = "These commands are supported:"
// )]
// enum Command {
//     Isbn(String),
//     Title(String),
//     Author(String),
// }

// impl From<Command> for Search {
//     fn from(command: Command) -> Self {
//         match command {
//             Command::Author(author) => Search::Author(author),
//             Command::Title(title) => Search::Title(title),
//             Command::Isbn(isbn) => Search::Isbn(isbn),
//         }
//     }
// }

// pub async fn callback_handler(
//     q: CallbackQuery,
//     bot: Bot,
//     utils: Arc<Utils>,
// ) -> Result<(), Box<dyn Error + Send + Sync>> {
//     let (user_id, chat_id) = match q.message {
//         Some(Message { id, chat, .. }) => (id, chat.id),
//         None => return Ok(()),
//     };

//     let ids = match q.data {
//         Some(id) => vec![id.parse().unwrap()],
//         None => {
//             bot.edit_message_text(chat_id, user_id, "💥").await?;
//             return Ok(());
//         }
//     };

//     let book = match get_ids(&utils.client, ids).await {
//         Ok(mut books) => books.remove(0),
//         Err(_) => {
//             bot.edit_message_text(chat_id, user_id, "💥").await?;
//             return Ok(());
//         }
//     };

//     utils.register(chat_id.0, user_id.0, "SELECTION")?;

//     let url_keyboard = make_url_keyboard(&book.md5_url());
//     bot.edit_message_text(chat_id, user_id, book.pretty())
//         .parse_mode(ParseMode::Html)
//         .reply_markup(url_keyboard)
//         .await?;

//     Ok(())
// }

async fn handle_image(
    bot: Bot,
    image: Option<&[PhotoSize]>,
    img_push: Arc<ImgPush>,
) -> Option<String> {
    let image = match image {
        Some(images) => images.iter().last(),
        None => None,
    };

    match image {
        Some(image) => {
            let file = bot.get_file(image.file.id.to_owned()).send().await;

            match file {
                Ok(res) => {
                    let tg_url = format!(
                        "https://api.telegram.org/file/bot{}/{}",
                        bot.token(),
                        &res.path
                    );
                    let image_url = img_push.upload(&tg_url).await.unwrap();

                    Some(image_url)
                }
                Err(err) => {
                    None
                }
            }
        }
        None => None,
    }
}

#[derive(Default)]
struct TextElements {
    pub title: Option<String>,
    pub url: Option<String>,
    pub tags: Option<Vec<String>>,
}

fn handle_text(bot: Bot, text: String) -> TextElements {
    let title = text.lines().next().unwrap_or(&text).to_string();

    let links_reg: Regex = Regex::new(r"(https?:\/\/[^\s]+)").unwrap();
    let links = match_regex(links_reg, &text);
    let first_link = links.and_then(|vec| vec.into_iter().next());

    let tags_reg: Regex = Regex::new(r"\s(?:@|#)(\w+)").unwrap();
    let tags = match_regex(tags_reg, &text);

    TextElements {
        title: Some(title),
        url: first_link,
        tags,
    }
}

pub async fn message_handler(
    bot: Bot,
    m: Message,
    notion: Arc<Notion>,
    img_push: Arc<ImgPush>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let text = m.text().unwrap_or("").to_string() + m.caption().unwrap_or("");

    let text_elements = handle_text(bot.clone(), text);
    let image_url = handle_image(bot.clone(), m.photo(), img_push).await;

    // TODO, read from dialogue
    let database_id = env::var("DATABASE_ID").expect("DATABASE_ID not set");
    let database = notion.get_database_by_id(database_id).await.unwrap();

    let new_page = NewPage {
        parent_database: database,
        name: text_elements.title,
        tags: text_elements.tags,
        url: text_elements.url,
        image_url,
    };

    let page = notion.create_page(new_page).await.unwrap();
    let page_id = page.id.to_string().replace("-", "");

    let msg = bot
        .send_message(
            m.chat.id,
            format!("Created page https://notion.so/{page_id}"),
        )
        .reply_to_message_id(m.id)
        .await?;

    Ok(())
}

fn match_regex(reg: Regex, text: &String) -> Option<Vec<String>> {
    let tags: Vec<String> = reg
        .captures_iter(&text)
        .map(|cap| cap.get(1).unwrap().as_str().to_string())
        .collect();

    if tags.len() != 0 {
        Some(tags)
    } else {
        None
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_match_tags() {
//         let tags = match_regex("hallo #beach #vacay\nok #lol".to_string());
//         assert_eq!(
//             tags,
//             Some(vec![
//                 "beach".to_string(),
//                 "vacay".to_string(),
//                 "lol".to_string()
//             ])
//         );
//     }
// }
