use serenity::client::Context;
use serenity::model::channel::Message;

pub fn log(_: &Context, msg: &Message, content: String) {
    if msg.is_private() {
        println!("[{}] {}", msg.author.tag(), content);
    } else {
        println!("[{}] {}", msg.guild_id.unwrap().0, content);
    }
}
