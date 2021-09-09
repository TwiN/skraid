use crate::config::{Config, KEY_MAINTAINER_ID};
use serenity::framework::standard::CommandError;
use serenity::model::channel::Message;
use serenity::model::id::UserId;
use serenity::{client::Context, framework::standard::CommandResult};

pub async fn forward_to_maintainer(ctx: &Context, msg: &Message) -> CommandResult {
    let maintainer_user_id_as_string: String;
    {
        let reader = ctx.data.read().await;
        let config = reader.get::<Config>().expect("Expected Config to exist in context data").clone();
        let cfg = config.read().unwrap();
        maintainer_user_id_as_string = cfg.get(KEY_MAINTAINER_ID).unwrap().clone();
    }
    let maintainer_user_id = match maintainer_user_id_as_string.parse::<u64>() {
        Ok(n) => n,
        Err(e) => return Err(CommandError::from(e.to_string())),
    };
    match UserId(maintainer_user_id).create_dm_channel(ctx).await {
        Ok(private_channel) => {
            let _ = private_channel
                .send_message(&ctx, |m| {
                    m.add_embed(|e| {
                        e.description(format!(
                            "Forwarded message from `{}` ({}) from Guild `{}`: ```{}```",
                            msg.author.tag(),
                            msg.author.id.0,
                            msg.guild_id.unwrap().0,
                            msg.content
                        ))
                    })
                })
                .await;
        }
        Err(e) => {
            eprintln!("Failed to send private message to maintainer: {}", e.to_string());
        }
    }
    return Ok(());
}
