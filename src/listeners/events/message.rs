use crate::utilities::logging::log;
use serenity::client::Context;
use serenity::model::channel::Message;

pub async fn message(ctx: Context, msg: Message) {
    // Ignore short messages, unlikely to be noteworthy
    if msg.content.len() < 15 {
        return;
    }
    if msg.content.contains("://diskord.") || msg.content.contains("://discorcl.") {
        log(&ctx, &msg, format!("userId={} posted suspicious message={}", msg.author.id.0, msg.content))
    }
}
