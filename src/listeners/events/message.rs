use crate::database::Database;
use crate::utilities::logging::log;
use serenity::client::Context;
use serenity::model::channel::Message;
use serenity::model::id::ChannelId;
use serenity::model::Permissions;

pub async fn message(ctx: Context, msg: Message) {
    if msg.author.bot || msg.is_private() {
        return;
    }
    // Ignore short messages, unlikely to be noteworthy
    if msg.content.len() < 15 {
        return;
    }
    let message_content = msg.content.to_lowercase();
    let contains_forbidden_word: bool;
    let mut alert_only: bool = false;
    let mut alert_channel_id: u64 = 0;
    {
        let data = ctx.data.read().await;
        let mutex = data.get::<Database>().unwrap();
        let db = mutex.lock().unwrap();
        contains_forbidden_word = match db.contains_forbidden_word(message_content.to_string()) {
            Ok(b) => b,
            Err(e) => {
                log(&ctx, &msg, format!("Failed to check if message contained forbidden word: {}", e.to_string()));
                false
            }
        };
        if contains_forbidden_word {
            match db.get_guild_configuration(msg.guild_id.unwrap().0) {
                Ok((a, b)) => {
                    alert_only = a;
                    alert_channel_id = b;
                    ()
                }
                Err(e) => {
                    eprintln!("[{}] Failed to retrieve guild configuration: {}", msg.guild_id.unwrap().0, e.to_string());
                    ()
                }
            }
        }
    }
    if contains_forbidden_word {
        let bot_user = ctx.cache.current_user().await;
        let bot_member = ctx.cache.member(msg.guild_id.unwrap().0, bot_user.id).await;
        let action: String;
        if !alert_only {
            if bot_member.unwrap().permissions(&ctx).await.unwrap().contains(Permissions::MANAGE_MESSAGES) {
                log(&ctx, &msg, format!("user={} ({}) posted a message containing a forbidden word; action=DELETE: {}", msg.author.tag(), msg.author.id.0, message_content));
                action = " and the message has been deleted.".to_string();
                let _ = msg.delete(&ctx).await;
            } else {
                log(
                    &ctx,
                    &msg,
                    format!(
                        "user={} ({}) posted a message containing a forbidden word; action=ALERT (missing MANAGE_MESSAGES permission): {}",
                        msg.author.tag(),
                        msg.author.id.0,
                        msg.content
                    ),
                );
                action = format!(
                    ", but the message was not deleted due to missing MANAGE_MESSAGES permission:\nhttps://discord.com/channels/{}/{}/{}",
                    msg.guild_id.unwrap().0,
                    msg.channel_id.0,
                    msg.id.0
                );
            }
        } else {
            log(&ctx, &msg, format!("user={} ({}) posted a message containing a forbidden word; action=ALERT: {}", msg.author.tag(), msg.author.id.0, message_content));
            action = format!(
                ", but no action was taken due to alert_only being set to true:\nhttps://discord.com/channels/{}/{}/{}",
                msg.guild_id.unwrap().0,
                msg.channel_id.0,
                msg.id.0
            );
        }
        if alert_channel_id != 0 {
            let _ = ChannelId(alert_channel_id)
                .send_message(&ctx, |m| {
                    m.add_embed(|e| e.description(format!("User <@{}> posted a message containing a forbidden word{}: ```{}```", msg.author.id.0, action, message_content)))
                })
                .await;
        } else {
            log(&ctx, &msg, "WARNING: Guild does not have alert_channel_id configured".into());
        }
    } else if message_content.contains("http")
        && (message_content.contains("free") || message_content.contains("nitro") || message_content.contains("skin") || message_content.contains("win"))
    {
        log(&ctx, &msg, format!("Potentially suspicious message: {}", msg.content.to_string()));
    }
}
