use serenity::client::Context;
use serenity::model::channel::Message;

pub async fn message(ctx: Context, msg: Message) {
    // XXX: detect keywords in messages?
    // ://diskord.
    // ://discorcl.
    // if msg.content.contains()
}
