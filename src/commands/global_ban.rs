use crate::database::Database;
use crate::utilities::logging::log;
use serenity::framework::standard::CommandError;
use serenity::model::channel::Message;
use serenity::model::channel::ReactionType::Unicode;
use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
};

#[command]
#[description("Add user ID to the global ban list")]
#[aliases(gban)]
#[min_args(1)]
async fn blocklist(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    log(ctx, msg, msg.content.to_string());
    let id = match args.single::<String>().unwrap().parse::<u64>() {
        Ok(n) => n,
        Err(e) => return Err(CommandError::from(e.to_string())),
    };
    let reason = args.rest();
    {
        let lock = ctx.data.read().await;
        let db = lock.get::<Database>().unwrap();
        match db.insert_in_blocklist(id, reason.to_string()) {
            Ok(_) => (),
            Err(e) => return Err(CommandError::from(e.to_string())),
        };
    }
    msg.react(ctx, Unicode("✅".into())).await?;
    log(ctx, msg, format!("Successfully added id={} to blacklist for reason={}", id, reason));
    return Ok(());
}

#[command]
#[description("Remove user ID from the global ban list")]
#[aliases(gunban)]
#[min_args(1)]
async fn unblocklist(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    log(ctx, msg, msg.content.to_string());
    let id = match args.rest().parse::<u64>() {
        Ok(n) => n,
        Err(e) => return Err(CommandError::from(e.to_string())),
    };
    {
        let lock = ctx.data.read().await;
        let db = lock.get::<Database>().unwrap();
        match db.remove_from_blocklist(id) {
            Ok(b) => b,
            Err(e) => return Err(CommandError::from(e.to_string())),
        };
    }
    msg.react(ctx, Unicode("✅".into())).await?;
    return Ok(());
}

#[command]
#[description("Check if a user ID is in the global ban list")]
#[min_args(1)]
async fn is_blocklisted(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    log(ctx, msg, msg.content.to_string());
    let id = match args.rest().parse::<u64>() {
        Ok(n) => n,
        Err(e) => return Err(CommandError::from(e.to_string())),
    };
    let is_banned: bool;
    {
        let lock = ctx.data.read().await;
        let db = lock.get::<Database>().unwrap();
        is_banned = match db.is_blocklisted(id) {
            Ok(b) => b,
            Err(e) => return Err(CommandError::from(e.to_string())),
        };
    }
    msg.reply(ctx, format!("{}", is_banned)).await?;
    msg.react(ctx, Unicode("✅".into())).await?;
    return Ok(());
}
