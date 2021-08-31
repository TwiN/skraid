use crate::config::{Config, KEY_MAINTAINER_ID};
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
    let ids = args.single::<String>().unwrap();
    let ids_iterator = ids.split(",");
    let reason = args.rest();
    {
        let data = ctx.data.read().await;
        let mutex = data.get::<Database>().unwrap();
        let db = mutex.lock().unwrap();
        for id in ids_iterator {
            let id = match id.parse::<u64>() {
                Ok(n) => n,
                Err(e) => return Err(CommandError::from(e.to_string())),
            };
            match db.insert_in_blocklist(id, reason.to_string()) {
                Ok(_) => (),
                Err(e) => return Err(CommandError::from(e.to_string())),
            };
            log(ctx, msg, format!("Successfully added id={} to blacklist for reason={}", id, reason));
        }
    }
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

#[command]
#[description("Suggest the addition of a user ID to the global blocklist to the maintainer.")]
#[usage("USER_ID REASON")]
#[example("000000000000000000 Scammer")]
#[min_args(2)]
#[bucket(suggestion)]
async fn suggest_blocklist(ctx: &Context, msg: &Message) -> CommandResult {
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
    let user = ctx.http.get_user(maintainer_user_id).await.unwrap();
    let _ = user.direct_message(&ctx, |m| m.content(format!("{} from {}: ```{}```", msg.author.tag(), msg.guild_id.unwrap().0, msg.content))).await;
    return Ok(());
}
