use teloxide::prelude::*;
use teloxide::{Bot, types::ChatId};

pub async fn send_alert(message: &str) {
    let bot_token = match std::env::var("BOT_TOKEN") {
        Ok(val) => val,
        Err(_) => {
            eprintln!("⚠️ BOT_TOKEN not set");
            return;
        }
    };

    let chat_id_raw = match std::env::var("CHAT_ID").ok().and_then(|id| id.parse::<i64>().ok()) {
        Some(id) => id,
        None => {
            eprintln!("⚠️ CHAT_ID not set or invalid");
            return;
        }
    };

    let chat_id = ChatId(chat_id_raw); // ✅ wrap i64 into ChatId
    let bot = Bot::new(bot_token);

    if let Err(err) = bot.send_message(chat_id, message).send().await {
        eprintln!("❌ Failed to send Telegram alert: {:?}", err);
    }
}


