use crate::antispam::{AntiSpam, ChannelAndMessageId};
use crate::database::Database;
use crate::utilities::logging::log;
use serenity::client::Context;
use serenity::model::channel::Message;
use serenity::model::id::{ChannelId, MessageId};
use serenity::model::Permissions;
use std::collections::HashMap;

pub async fn message(ctx: Context, msg: Message) {
    if msg.author.bot || msg.is_private() {
        return;
    }
    let mut is_spamming: bool = false;
    let mut messages_to_delete: Option<Vec<ChannelAndMessageId>> = None;
    let mut alert_only: bool = false;
    let mut alert_channel_id: u64 = 0;
    {
        let data = ctx.data.read().await;
        if let Some(db_mutex) = data.get::<Database>() {
            let db = db_mutex.lock().unwrap();
            match db.get_guild_configuration(msg.guild_id.unwrap().0) {
                Ok((a, b, _, _)) => {
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
        if let Some(anti_spam_mutex) = data.get::<AntiSpam>() {
            let mut anti_spam = anti_spam_mutex.lock().unwrap();
            is_spamming = anti_spam.check_if_spamming(msg.guild_id.unwrap().0, msg.author.id.0, msg.channel_id.0, msg.id.0);
            if is_spamming {
                log(&ctx, &msg, format!("User {} may be spamming: {}", msg.author.tag(), msg.content));
                if let Some(channel_and_message_ids) = anti_spam.get_recent_message_ids(msg.guild_id.unwrap().0, msg.author.id.0) {
                    messages_to_delete = Some(channel_and_message_ids.to_vec());
                }
                anti_spam.delete_recent_message_ids(msg.guild_id.unwrap().0, msg.author.id.0);
            }
        }
    }
    if is_spamming {
        handle_spammer(&ctx, &msg, &mut messages_to_delete, alert_only, alert_channel_id).await;
        return;
    }
    // Ignore short messages, unlikely to be noteworthy
    if msg.content.len() < 15 {
        return;
    }
    let message_content = msg.content.to_lowercase();
    let mut contains_forbidden_word: bool = false;
    {
        let data = ctx.data.read().await;
        if let Some(mutex) = data.get::<Database>() {
            let db = mutex.lock().unwrap();
            contains_forbidden_word = match db.contains_word_in_word_blocklist(message_content.to_string()) {
                Ok(b) => b,
                Err(e) => {
                    log(&ctx, &msg, format!("Failed to check if message contained forbidden word: {}", e.to_string()));
                    false
                }
            };
        }
    }
    if contains_forbidden_word {
        let bot_user = ctx.cache.current_user().await;
        let bot_member = ctx.cache.member(msg.guild_id.unwrap().0, bot_user.id).await;
        let action: String;
        if !alert_only {
            if bot_member.unwrap().permissions(&ctx).await.unwrap().contains(Permissions::MANAGE_MESSAGES) {
                action = " and the message has been deleted".to_string();
                let _ = msg.delete(&ctx).await;
            } else {
                action = ", but it was not deleted due to missing MANAGE_MESSAGES permission".into();
            }
        } else {
            action = ", but no action was taken due to being in alert-only mode".into();
        }
        log(&ctx, &msg, format!("User {} ({}) posted a message containing a forbidden word{}: {}", msg.author.tag(), msg.author.id.0, action, message_content));
        if alert_channel_id != 0 {
            let _ = ChannelId(alert_channel_id)
                .send_message(&ctx, |m| {
                    m.add_embed(|e| {
                        e.description(format!(
                            "<@{0}> posted a [message](https://discord.com/channels/{1}/{2}/{3}) in <#{2}> containing a forbidden word{4}: ```{5}```",
                            msg.author.id.0,
                            msg.guild_id.unwrap().0,
                            msg.channel_id.0,
                            msg.id.0,
                            action,
                            message_content
                        ))
                    })
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

async fn handle_spammer(ctx: &Context, msg: &Message, messages_to_delete: &mut Option<Vec<ChannelAndMessageId>>, alert_only: bool, alert_channel_id: u64) {
    if alert_only {
        if alert_channel_id != 0 {
            let _ = ChannelId(alert_channel_id)
                .send_message(&ctx, |m| {
                    m.add_embed(|e| {
                        e.description(format!("<@{0}> is currently spamming, but their messages will not be deleted due to `alert_only` being set to `true`", msg.author.id.0,))
                    })
                })
                .await;
        }
    } else {
        if let Some(messages) = messages_to_delete {
            let mut messages_by_channel: HashMap<u64, Vec<MessageId>> = HashMap::new();
            // split messages by channel so we can bulk delete them
            for message_to_delete in messages {
                if messages_by_channel.contains_key(&message_to_delete.channel_id) {
                    if let Some(messages_in_channel) = messages_by_channel.get_mut(&message_to_delete.channel_id) {
                        messages_in_channel.push(MessageId(message_to_delete.message_id));
                    }
                } else {
                    messages_by_channel.insert(message_to_delete.channel_id, vec![MessageId(message_to_delete.message_id)]);
                }
            }
            for (channel_id, message_ids) in messages_by_channel.into_iter() {
                //println!("{} {}", channel_id, message_ids.to_vec().into_iter().map(|i| i.to_string()).collect::<String>());
                match ChannelId(channel_id).delete_messages(&ctx, message_ids).await {
                    Ok(_) => (),
                    Err(e) => eprintln!("{}", e.to_string()),
                }
            }
        }
        if alert_channel_id != 0 {
            let _ = ChannelId(alert_channel_id)
                .send_message(&ctx, |m| m.add_embed(|e| e.description(format!("<@{0}> was spamming, so their messages were automatically deleted", msg.author.id.0,))))
                .await;
        }
    }
}
