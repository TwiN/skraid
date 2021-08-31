use crate::database::Database;
use crate::utilities::logging::log;
use serenity::model::channel::Message;
use serenity::model::id::ChannelId;
use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandError, CommandResult},
};

#[command]
#[description("Set the channel where the alerts will be sent")]
#[usage("CHANNEL_ID")]
#[example("000000000000000000")]
#[min_args(1)]
#[max_args(1)]
#[bucket(staff)]
async fn set_alert_channel(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let arguments = args.rest();
    let alert_channel_id = match arguments.parse::<u64>() {
        Ok(n) => n,
        Err(e) => return Err(CommandError::from(e.to_string())),
    };
    let verification_message = match ChannelId(alert_channel_id).send_message(ctx, |m| m.content("Checking if I can send messages in the alert channel..")).await {
        Ok(message) => message,
        Err(e) => return Err(CommandError::from(e.to_string())),
    };
    let _ = verification_message.delete(ctx).await;
    {
        let data = ctx.data.read().await;
        let mutex = data.get::<Database>().unwrap();
        let db = mutex.lock().unwrap();
        match db.upsert_guild_alert_channel_id(msg.guild_id.unwrap().0, alert_channel_id) {
            Ok(_) => log(ctx, msg, format!("Updated alert_channel_id to {}", alert_channel_id)),
            Err(e) => return Err(CommandError::from(e.to_string())),
        }
    }
    Ok(())
}

#[command]
#[description("Configure whether Skraid should only send alert without taking any action.")]
#[usage("BOOLEAN")]
#[example("true")]
#[min_args(1)]
#[max_args(1)]
#[bucket(staff)]
async fn set_alert_only(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let value = args.rest();
    let alert_only: bool;
    if value == "true" {
        alert_only = true;
    } else if value == "false" {
        alert_only = false;
    } else {
        return Err(CommandError::from("Value must be true or false"));
    }
    {
        let data = ctx.data.read().await;
        let mutex = data.get::<Database>().unwrap();
        let db = mutex.lock().unwrap();
        match db.upsert_guild_alert_only(msg.guild_id.unwrap().0, alert_only) {
            Ok(_) => log(ctx, msg, format!("Updated alert_only to {}", alert_only)),
            Err(e) => return Err(CommandError::from(e.to_string())),
        }
    }
    Ok(())
}

#[command]
#[description("Retrieve the current guild configuration.")]
#[min_args(0)]
#[max_args(0)]
#[bucket(staff)]
async fn get_guild_config(ctx: &Context, msg: &Message) -> CommandResult {
    let alert_only: bool;
    let alert_channel_id: u64;
    {
        let data = ctx.data.read().await;
        let mutex = data.get::<Database>().unwrap();
        let db = mutex.lock().unwrap();
        match db.get_guild_configuration(msg.guild_id.unwrap().0) {
            Ok((a, b)) => {
                alert_only = a;
                alert_channel_id = b;
                ()
            }
            Err(e) => return Err(CommandError::from(e.to_string())),
        }
    }
    let _ = msg.reply(ctx, format!("**alert_only:** {}\n**alert_channel_id:** <#{}>", alert_only, alert_channel_id)).await;
    Ok(())
}
