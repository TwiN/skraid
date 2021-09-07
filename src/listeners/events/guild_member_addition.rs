use crate::database::Database;
use chrono::Utc;
use serenity::client::Context;
use serenity::model::guild::Member;
use serenity::model::id::{ChannelId, GuildId};
use serenity::model::Permissions;

pub async fn guild_member_addition(ctx: Context, guild_id: GuildId, new_member: Member) {
    if new_member.user.bot {
        return;
    }
    let number_of_hours_since_account_creation = (Utc::now().timestamp() - new_member.user.created_at().timestamp()) / 3600;
    let was_account_created_recently = number_of_hours_since_account_creation <= 2;
    println!("[{}] {} ({}) joined {}; new?={}", guild_id.0, new_member.user.tag(), new_member.user.id.0, guild_id.name(&ctx).await.unwrap(), was_account_created_recently);
    let is_blocklisted: bool;
    let mut alert_only: bool = true;
    let mut alert_channel_id: u64 = 0;
    let mut ban_new_user_on_join: bool = false;
    let mut ban_user_on_join: bool = false;
    {
        let data = ctx.data.read().await;
        let mutex = data.get::<Database>().unwrap();
        let db = mutex.lock().unwrap();
        let is_allowlisted = match db.is_allowlisted(guild_id.0, new_member.user.id.0) {
            Ok(b) => b,
            Err(e) => {
                eprintln!("[{}] Failed to check whether user {} () was allowlisted: {}", guild_id.0, new_member.user.id.0, e.to_string());
                false
            }
        };
        if is_allowlisted {
            println!("[{}] {} ({}) is allowlisted", guild_id.0, new_member.user.tag(), new_member.user.id.0);
            return;
        }
        is_blocklisted = match db.is_blocklisted(new_member.user.id.0) {
            Ok(b) => b,
            Err(e) => {
                eprintln!("[{}] Failed to check whether user {} was blocklisted: {}", guild_id.0, new_member.user.id.0, e.to_string());
                false
            }
        };
        match db.get_guild_configuration(guild_id.0) {
            Ok((a, b, c, d)) => {
                alert_only = a;
                alert_channel_id = b;
                ban_new_user_on_join = c;
                ban_user_on_join = d;
                ()
            }
            Err(e) => {
                eprintln!("[{}] Failed to retrieve guild configuration: {}", guild_id.0, e.to_string());
                ()
            }
        }
    }
    if is_blocklisted || (was_account_created_recently && ban_new_user_on_join) || ban_user_on_join {
        let bot_user = ctx.cache.current_user().await;
        let bot_member = ctx.cache.member(guild_id, bot_user.id).await;
        let description: &str;
        if !alert_only {
            if bot_member.unwrap().permissions(&ctx).await.unwrap().contains(Permissions::BAN_MEMBERS) {
                if ban_user_on_join {
                    description = "has been banned because ban_user_on_join is set to true";
                    let _ = new_member.ban_with_reason(&ctx, 0, "Skraid: ban_user_on_join").await;
                } else if ban_new_user_on_join && was_account_created_recently {
                    description = "has been banned for being a new account";
                    let _ = new_member.ban_with_reason(&ctx, 0, "Skraid: ban_new_user_on_join").await;
                } else {
                    description = "has been banned for being in the global blocklist";
                    let _ = new_member.ban_with_reason(&ctx, 0, "Skraid: global blocklist").await;
                }
            } else {
                if ban_user_on_join {
                    description = "should've been banned because ban_user_on_join is set to true, but was not due to missing BAN_MEMBERS permissions";
                } else if ban_new_user_on_join && was_account_created_recently {
                    description = "is a new account, but was not banned due to missing BAN_MEMBERS permissions";
                } else {
                    description = "is in the global blocklist, but was not banned due to missing BAN_MEMBERS permissions";
                }
            }
        } else {
            if ban_user_on_join {
                description = "should've been banned because ban_user_on_join is set to true, but no action was taken due to alert_only being set to true";
            } else if ban_new_user_on_join && was_account_created_recently {
                description = "is a new account, but no action was taken due to alert_only being set to true";
            } else {
                description = "is in the global blocklist, but no action was taken due to alert_only being set to true";
            }
        }
        println!("[{}] User {} {}", guild_id.0, new_member.user.tag(), description);
        if alert_channel_id != 0 {
            let _ = ChannelId(alert_channel_id).send_message(&ctx, |m| m.add_embed(|e| e.description(format!("User <@{}> {}", new_member.user.id.0, description)))).await;
        } else {
            println!("[{}] WARNING: Guild does not have alert_channel_id configured", guild_id.0);
        }
    }
}
