use serenity::client::Context;
use serenity::model::channel::Message;

pub async fn message(ctx: Context, msg: Message) {
    // Ignore short messages, unlikely to be noteworthy
    if msg.content.len() < 15 {
        return;
    }
    if msg.content.contains("://diskord.") || msg.content.contains("://discorcl.") {
        println!("[{}] userId={} posted suspicious message={}", ctx.cache.guild(msg.guild_id.unwrap().0).await.unwrap().name, msg.author.id.0, msg.content)
    }
}
