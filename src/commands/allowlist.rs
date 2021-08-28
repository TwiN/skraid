use crate::database::Database;
use crate::utilities::logging::log;
use serenity::framework::standard::CommandError;
use serenity::model::channel::Message;
use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
};

#[command]
#[description("Add user ID to the guild's list of exception (allowlist). This is only necessary if Skraid banned a user that you believe is legitimate.")]
#[usage("USER_ID")]
#[example("000000000000000000")]
#[aliases(gban)]
#[min_args(1)]
async fn allowlist(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let user_id = match args.single::<String>().unwrap().parse::<u64>() {
        Ok(n) => n,
        Err(e) => return Err(CommandError::from(e.to_string())),
    };
    {
        let lock = ctx.data.read().await;
        let db = lock.get::<Database>().unwrap();
        match db.insert_in_allowlist(msg.guild_id.unwrap().0, user_id) {
            Ok(_) => (),
            Err(e) => return Err(CommandError::from(e.to_string())),
        };
    }
    log(ctx, msg, format!("Added id={} to allowlist", user_id));
    return Ok(());
}

#[command]
#[description("Remove user ID from the guild's list of exception (allowlist)")]
#[usage("USER_ID")]
#[example("000000000000000000")]
#[aliases(gunban)]
#[min_args(1)]
async fn unallowlist(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let user_id = match args.rest().parse::<u64>() {
        Ok(n) => n,
        Err(e) => return Err(CommandError::from(e.to_string())),
    };
    {
        let lock = ctx.data.read().await;
        let db = lock.get::<Database>().unwrap();
        match db.remove_from_allowlist(msg.guild_id.unwrap().0, user_id) {
            Ok(b) => b,
            Err(e) => return Err(CommandError::from(e.to_string())),
        };
    }
    return Ok(());
}

#[command]
#[description("Check if a user ID is in the guild's list of exception (allowlist)")]
#[usage("USER_ID")]
#[example("000000000000000000")]
#[min_args(1)]
async fn is_allowlisted(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let user_id = match args.rest().parse::<u64>() {
        Ok(n) => n,
        Err(e) => return Err(CommandError::from(e.to_string())),
    };
    let is_allowlisted: bool;
    {
        let lock = ctx.data.read().await;
        let db = lock.get::<Database>().unwrap();
        is_allowlisted = match db.is_allowlisted(msg.guild_id.unwrap().0, user_id) {
            Ok(b) => b,
            Err(e) => return Err(CommandError::from(e.to_string())),
        };
    }
    msg.reply(ctx, format!("{}", is_allowlisted)).await?;
    return Ok(());
}

#[command]
#[description("Retrieves a list of all allowlisted user ids for this guild")]
async fn get_allowlisted_users(ctx: &Context, msg: &Message) -> CommandResult {
    let allowlisted_users: Vec<u64>;
    {
        let lock = ctx.data.read().await;
        let db = lock.get::<Database>().unwrap();
        allowlisted_users = match db.get_allowlisted_users_in_guild(msg.guild_id.unwrap().0) {
            Ok(users) => users,
            Err(e) => return Err(CommandError::from(e.to_string())),
        };
    }
    if allowlisted_users.is_empty() {
        msg.reply(ctx, "There is currently no users in the allowlist for this guild.").await?;
    } else {
        let mut message: String = "".to_owned();
        for allowlisted_user in allowlisted_users {
            message.push_str(allowlisted_user.to_string().as_str());
            message.push_str("\n");
        }
        println!("{}", message);
        msg.reply(ctx, format!("**List of allowlisted users in this guild**:\n```\n{}```", message)).await?;
    }
    return Ok(());
}
