use crate::config::{Config, KEY_PREFIX};
use serenity::client::Context;
use serenity::model::gateway::{Activity, Ready};
use serenity::model::user::OnlineStatus::Online;

pub async fn ready(ctx: Context, ready: Ready) {
    println!("{} is connected!", ready.user.name);
    let command: String;
    {
        let reader = ctx.data.read().await;
        let config = reader.get::<Config>().expect("Expected Config to exist in context data").clone();
        let cfg = config.read().unwrap();
        let prefix = cfg.get(KEY_PREFIX).unwrap();
        command = format!("{}help", prefix);
    }
    ctx.set_presence(Some(Activity::listening(command)), Online).await;
}
