use crate::database::Database;
use serenity::client::Context;
use serenity::model::guild::Member;
use serenity::model::id::{ChannelId, GuildId};
use serenity::model::Permissions;

pub async fn guild_member_addition(ctx: Context, guild_id: GuildId, new_member: Member) {
    if new_member.user.bot {
        return;
    }
    println!("[{}] {} ({}) joined {}", guild_id.0, new_member.user.tag(), new_member.user.id.0, guild_id.name(&ctx).await.unwrap());
    let is_blocklisted: bool;
    let mut is_allowlisted: bool = false;
    let mut alert_only: bool = false;
    let mut alert_channel_id: u64 = 0;
    {
        let data = ctx.data.read().await;
        let mutex = data.get::<Database>().unwrap();
        let db = mutex.lock().unwrap();
        is_blocklisted = match db.is_blocklisted(new_member.user.id.0) {
            Ok(b) => b,
            Err(e) => {
                eprintln!("[{}] Failed to check whether user {} was blocklisted: {}", guild_id.0, new_member.user.id.0, e.to_string());
                false
            }
        };
        if is_blocklisted {
            is_allowlisted = match db.is_allowlisted(guild_id.0, new_member.user.id.0) {
                Ok(b) => b,
                Err(e) => {
                    eprintln!("[{}] Failed to check whether user {} () was allowlisted: {}", guild_id.0, new_member.user.id.0, e.to_string());
                    false
                }
            };
            // Since it's blocklisted and not allowlisted, we need to get the guild configuration
            if !is_allowlisted {
                match db.get_guild_configuration(guild_id.0) {
                    Ok((a, b)) => {
                        alert_only = a;
                        alert_channel_id = b;
                        ()
                    }
                    Err(e) => {
                        eprintln!("[{}] Failed to retrieve guild configuration: {}", guild_id.0, e.to_string());
                        ()
                    }
                }
            }
        }
    }
    if is_blocklisted && !is_allowlisted {
        let bot_user = ctx.cache.current_user().await;
        let bot_member = ctx.cache.member(guild_id, bot_user.id).await;
        let action: &str;
        if !alert_only {
            if bot_member.unwrap().permissions(&ctx).await.unwrap().contains(Permissions::BAN_MEMBERS) {
                println!("[{}] user={} is in global blocklist and not allowlisted; action=BAN", guild_id.0, new_member.user.tag());
                action = " and has been banned";
                let _ = new_member.ban_with_reason(&ctx, 0, "Skraid global blocklist").await;
            } else {
                println!("[{}] user={} is in global blocklist and not allowlisted; action=ALERT", guild_id.0, new_member.user.tag());
                action = ", but was not banned due to missing BAN_MEMBERS permissions";
            }
        } else {
            println!("[{}] user={} is in global blocklist and not allowlisted; action=ALERT", guild_id.0, new_member.user.tag());
            action = ", but no action was taken due to alert_only being set to true";
        }
        if alert_channel_id != 0 {
            let _ = ChannelId(alert_channel_id)
                .send_message(&ctx, |m| m.add_embed(|e| e.description(format!("User <@{}> is in the global blocklist{}", new_member.user.id.0, action))))
                .await;
        } else {
            println!("[{}] WARNING: Guild does not have alert_channel_id configured", guild_id.0);
        }
    }
}
