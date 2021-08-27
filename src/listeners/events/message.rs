use serenity::client::Context;
use serenity::model::channel::Message;

pub async fn message(_: Context, _: Message) {
    // XXX: detect keywords in messages?
    // ://diskord.
    // ://discorcl.
    // if msg.content.contains()
}
