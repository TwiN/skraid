use crate::database::Database;
use crate::utilities::logging::log;
use serenity::client::Context;
use serenity::model::channel::Message;

pub async fn message(ctx: Context, msg: Message) {
    if msg.author.bot {
        return;
    }
    // Ignore short messages, unlikely to be noteworthy
    if msg.content.len() < 15 {
        return;
    }
    let contains_forbidden_word: bool;
    {
        let data = ctx.data.read().await;
        let mutex = data.get::<Database>().unwrap();
        let db = mutex.lock().unwrap();
        contains_forbidden_word = match db.contains_forbidden_word(msg.content.to_string()) {
            Ok(b) => b,
            Err(e) => {
                log(&ctx, &msg, format!("Failed to check if message contained forbidden word: {}", e.to_string()));
                false
            }
        };
    }
    if contains_forbidden_word {
        log(&ctx, &msg, format!("userId={} posted message containing a forbidden word: {}", msg.author.id.0, msg.content))
    }
}
