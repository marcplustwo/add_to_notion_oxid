use crate::db::Database;
use crate::img_push::ImgPush;
use crate::notion::{NewPage, Notion};
use regex::Regex;
use std::error::Error;
use std::sync::Arc;
use teloxide::prelude::*;
use teloxide::types::{Document, PhotoSize};

fn get_image_id(image: Option<&[PhotoSize]>) -> Option<String> {
    let image = match image {
        Some(images) => images.iter().last(),
        None => None,
    };

    match image {
        Some(image) => Some(image.file.id.to_owned()),
        None => None,
    }
}

fn get_document_id(document: Option<&Document>) -> Option<String> {
    if let Some(document) = document {
        if let Some(mime) = &document.mime_type {
            match (mime.type_(), mime.subtype()) {
                (mime::IMAGE, mime::JPEG) => Some(document.file.id.to_owned()),
                (mime::IMAGE, mime::PNG) => Some(document.file.id.to_owned()),
                _ => None,
            }
        } else {
            None
        }
    } else {
        None
    }
}

async fn upload_image_file(bot: Bot, img_push: Arc<ImgPush>, file_id: String) -> Option<String> {
    let file = bot.get_file(file_id.to_owned()).send().await;

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
        Err(_) => None,
    }
}

#[derive(Default)]
struct TextElements {
    pub title: Option<String>,
    pub url: Option<String>,
    pub tags: Option<Vec<String>>,
}

fn handle_text(text: String) -> TextElements {
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
    msg: Message,
    db: Arc<Database>,
    img_push: Arc<ImgPush>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let user_details_query = db.get(&msg.chat.id.to_string())?;
    let user_details = user_details_query.unwrap();

    let text = msg.text().unwrap_or("").to_string() + msg.caption().unwrap_or("");

    let text_elements = handle_text(text);
    let image_file_id = get_image_id(msg.photo());
    let document_file_id = get_document_id(msg.document());

    let mut urls = vec![];
    if let Some(id) = image_file_id {
        if let Some(url) = upload_image_file(bot.clone(), img_push.clone(), id).await {
            urls.push(url);
        }
    };
    if let Some(id) = document_file_id {
        if let Some(url) = upload_image_file(bot.clone(), img_push.clone(), id).await {
            urls.push(url);
        }
    };

    let image_url = urls.iter().next();

    let notion = Notion::new(user_details.integration_token);
    let database = notion
        .get_database_by_id(user_details.database_id)
        .await
        .unwrap();

    if !Notion::has_expected_database_properties(&database) {
        let fields = database.properties.keys().collect::<Vec<&String>>();
        let error_message = format!(
            "Database does not have all required fields: Name, Image, URL, Tags, found: {fields:?}"
        );

        bot.send_message(
            msg.chat.id,
            "Could not create page in Notion: ".to_string() + &error_message,
        )
        .await?;

        return Err(error_message.into());
    };

    let new_page = NewPage {
        parent_database: database,
        name: text_elements.title,
        tags: text_elements.tags,
        url: text_elements.url,
        image_url: image_url.cloned(),
    };
    let page = notion.create_page(new_page).await.unwrap();
    let page_id = page.id.to_string().replace("-", "");

    bot.send_message(
        msg.chat.id,
        format!("Created page https://notion.so/{page_id}"),
    )
    .reply_to_message_id(msg.id)
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
