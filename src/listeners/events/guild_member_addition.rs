use crate::antiraid::AntiRaid;
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
    println!("[{}] {} ({}) joined {}; new_account={}", guild_id.0, new_member.user.tag(), new_member.user.id.0, guild_id.name(&ctx).await.unwrap(), was_account_created_recently);
    let mut is_blocklisted: bool = false;
    let mut is_raiding: bool = false;
    let mut users_to_ban_if_is_raiding: Option<Vec<u64>> = None;
    let mut alert_only: bool = true;
    let mut alert_channel_id: u64 = 0;
    let mut ban_new_user_on_join: bool = false;
    let mut ban_user_on_join: bool = false;
    {
        let data = ctx.data.read().await;
        if let Some(db_mutex) = data.get::<Database>() {
            let db = db_mutex.lock().unwrap();
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
            is_blocklisted = match db.is_in_user_blocklist(new_member.user.id.0) {
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
        if let Some(anti_raid_mutex) = data.get::<AntiRaid>() {
            let mut anti_raid = anti_raid_mutex.lock().unwrap();
            is_raiding = anti_raid.check_if_raiding(guild_id.0, new_member.user.id.0);
            if is_raiding {
                if let Some(recent_member_ids) = anti_raid.get_recent_member_ids(guild_id.0) {
                    users_to_ban_if_is_raiding = Some(recent_member_ids.to_vec());
                    anti_raid.delete_recent_member_ids(guild_id.0);
                }
            }
        }
    }
    if is_raiding {
        if let Some(users_to_ban) = users_to_ban_if_is_raiding {
            if alert_channel_id != 0 {
                let _ = ChannelId(alert_channel_id).send_message(&ctx, |m| {
                    m.add_embed(|e| {
                        e.title("Anti-raid (ALPHA)");
                        e.description("At least 5 users have joined within the last 10 seconds\n\nYour guild may currently be getting raided, but due to the anti-raid feature being in ALPHA, no automated action will be taken.\n\nIf you really are being raided, using the command `SetBanUserOnJoin true` will automatically ban users attempting to join your guild.");
                        e
                    })
                }).await;
            }
            println!(
                "[{}] Guild may currently be getting raided. (DISABLED BECAUSE ALPHA) Users to ban: {}",
                guild_id.0,
                users_to_ban.to_vec().into_iter().map(|i| i.to_string()).collect::<String>()
            );
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
