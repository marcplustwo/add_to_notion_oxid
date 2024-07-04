use bot::run_bot;
use db::Database;
use dotenvy::dotenv;
use img_push::ImgPush;
use std::env;
use std::sync::Arc;

mod bot;
mod constants;
mod db;
mod handlers;
mod img_push;
mod notion;

#[tokio::main]
async fn main() -> Result<(), String> {
    println!("Starting the bot!");

    pretty_env_logger::init();

    if env::var("IMG_PUSH_URL").is_err() || env::var("TELOXIDE_TOKEN").is_err() {
        dotenv().expect(".env file not found");
    }

    let img_push_url = env::var("IMG_PUSH_URL").expect("IMG_PUSH_URL not set");
    env::var("TELOXIDE_TOKEN").expect("TELOXIDE_TOKEN not set");
    let db_path = env::var("DB_PATH").unwrap_or("db/db.sqlite".to_string());

    let db = Arc::new(Database::new(&db_path).unwrap());
    let img_push = Arc::new(ImgPush::new(img_push_url));

    run_bot(db, img_push).await;

    log::info!("Closing bot... Goodbye!");

    Ok(())
}
