use crate::database::Database;
use crate::utilities::logging::log;
use serenity::framework::standard::CommandError;
use serenity::model::channel::Message;
use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
};

#[command]
#[description("Add user ID to the global ban list")]
#[usage("USER_ID [REASON]")]
#[example("000000000000000000 Scam")]
#[aliases(gban)]
#[min_args(1)]
async fn blocklist(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let id = match args.single::<String>().unwrap().parse::<u64>() {
        Ok(n) => n,
        Err(e) => return Err(CommandError::from(e.to_string())),
    };
    let reason = args.rest();
    {
        let data = ctx.data.read().await;
        let mutex = data.get::<Database>().unwrap();
        let db = mutex.lock().unwrap();
        match db.insert_in_blocklist(id, reason.to_string()) {
            Ok(_) => (),
            Err(e) => return Err(CommandError::from(e.to_string())),
        };
    }
    log(ctx, msg, format!("Successfully added id={} to blacklist for reason={}", id, reason));
    return Ok(());
}

#[command]
#[description("Remove user ID from the global ban list")]
#[usage("USER_ID")]
#[example("000000000000000000")]
#[aliases(gunban)]
#[min_args(1)]
async fn unblocklist(ctx: &Context, _: &Message, args: Args) -> CommandResult {
    let id = match args.rest().parse::<u64>() {
        Ok(n) => n,
        Err(e) => return Err(CommandError::from(e.to_string())),
    };
    {
        let data = ctx.data.read().await;
        let mutex = data.get::<Database>().unwrap();
        let db = mutex.lock().unwrap();
        match db.remove_from_blocklist(id) {
            Ok(b) => b,
            Err(e) => return Err(CommandError::from(e.to_string())),
        };
    }
    return Ok(());
}

#[command]
#[description("Check if a user ID is in the global ban list")]
#[usage("USER_ID")]
#[example("000000000000000000")]
#[min_args(1)]
async fn is_blocklisted(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let id = match args.rest().parse::<u64>() {
        Ok(n) => n,
        Err(e) => return Err(CommandError::from(e.to_string())),
    };
    let is_banned: bool;
    {
        let data = ctx.data.read().await;
        let mutex = data.get::<Database>().unwrap();
        let db = mutex.lock().unwrap();
        is_banned = match db.is_blocklisted(id) {
            Ok(b) => b,
            Err(e) => return Err(CommandError::from(e.to_string())),
        };
    }
    msg.reply(ctx, format!("{}", is_banned)).await?;
    return Ok(());
}
