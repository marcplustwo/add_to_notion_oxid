use std::env;
use std::{error::Error, sync::Arc};

use notion::{NewPage, Notion};
use regex::Regex;
use teloxide::net::Download;
use teloxide::payloads::{EditMessageTextSetters, GetFile};
use teloxide::types::ParseMode;
use teloxide::utils::html::link;
use teloxide::{prelude::*, utils::command::BotCommands};
use tokio::fs;

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

pub async fn message_handler(
    bot: Bot,
    m: Message,
    notion: Arc<Notion>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let chat_id = m.chat.id;

    // let image = match m.photo() {
    //     Some(images) => images.iter().last(),
    //     None => None,
    // };

    // if let Some(image) = image {
    //     // let get_file = GetFile::new(image.file.id.to_owned());
    //     if let Ok(file) = bot.get_file(image.file.id.to_owned()).send().await {
    //         let mut dst = fs::File::create("/tmp/test.png").await.unwrap();
    //         bot.download_file(&file.path, &mut dst).await.unwrap();
    //     }
    // }

    let text = m.text().unwrap().to_string();

    let name = text.lines().next().unwrap_or(&text).to_string();

    let links_reg: Regex = Regex::new(r"(https?:\/\/[^\s]+)").unwrap();
    let links = match_regex(links_reg, &text);
    let first_link = links.and_then(|vec| vec.into_iter().next());

    let tags_reg: Regex = Regex::new(r"\s(?:@|#)(\w+)").unwrap();
    let tags = match_regex(tags_reg, &text);

    let database_id = env::var("DATABASE_ID").expect("DATABASE_ID not set");
    let database = notion.get_database_by_id(database_id).await.unwrap();

    let new_page = NewPage {
        parent_database: database,
        name,
        tags,
        url: first_link,
        image_url: None,
    };

    let page = notion.create_page(new_page).await.unwrap();
    let page_id = page.id.to_string();

    let msg = bot
        .send_message(
            chat_id,
            format!(
                "Created page https://notion.so/{}",
                page_id.replace("-", "")
            ),
        )
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
