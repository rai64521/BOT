use teloxide::prelude::*;
use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok(); 

    pretty_env_logger::init();
    // log 
    log::info!("Telegram bot...");

    let bot = Bot::from_env().auto_send();

    teloxide::repl(bot, |message| async move {
        message.answer("Hey, there i am sumant rai!").send().await?;
        Ok(())
    })
    .await;
}
