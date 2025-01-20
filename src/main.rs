use teloxide::prelude::*;
use dotenv::dotenv;
use std::env;
use log;

use solana_allstars_ve_bot::{Command, answer};

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Iniciando bot...");

    dotenv().ok();
    
    let token = env::var("TELEGRAM_BOT_TOKEN")
        .expect("TELEGRAM_BOT_TOKEN debe estar configurado en el archivo .env");
    
    let bot = Bot::new(token);

    Command::repl(bot, answer).await;
}
