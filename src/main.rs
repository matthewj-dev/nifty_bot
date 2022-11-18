extern crate pretty_env_logger;
#[macro_use] extern crate log;

use teloxide::prelude::*;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    info!("Starting throw dice bot...");

    let bot = Bot::from_env();

    teloxide::repl(bot, |bot: Bot, msg: Message| async move {
        let message_content = msg.text().expect("Error receiving message content!");
        let chat_id = msg.chat.id;

        info!("Sending Response to \"{message_content}!\"");

        let response_message = match message_content {
            "Hello there" => "General Kenobi!",
            "General Kenobi!" => "NOOOOOOOOOOOOO",
            _ => "Hello there"
        };
        bot.send_message(chat_id, response_message).await?;

        info!("Response Sent!");

        Ok(())
    })
    .await;
}
