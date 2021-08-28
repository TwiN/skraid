use crate::database::Database;
use serenity::framework::standard::CommandError;
use serenity::model::channel::Message;
use serenity::model::channel::ReactionType::Unicode;
use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
};

#[command]
#[description("Add user ID to global ban list")]
#[aliases(gban)]
#[min_args(1)]
async fn global_ban(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    if msg.is_private() {
        println!("[{}#{}] {}", msg.author.name, msg.author.discriminator, msg.content.as_str());
    } else {
        println!("[{}] {}", ctx.cache.guild(msg.guild_id.unwrap().0).await.unwrap().name, msg.content.as_str());
    }
    let lock = ctx.data.read().await;
    let db = lock.get::<Database>().unwrap();
    let id = match args.rest().parse::<u64>() {
        Ok(n) => n,
        Err(e) => return Err(CommandError::from(e.to_string())),
    };
    match db.insert_banned_user(id) {
        Ok(_) => (),
        Err(e) => return Err(CommandError::from(e.to_string())),
    };
    msg.react(ctx, Unicode("✅".to_string())).await?;
    println!("");
    return Ok(());
}

#[command]
#[description("Check if a user ID is in the global ban list")]
#[aliases(cban, isbanned, checkban)]
#[min_args(1)]
async fn is_global_ban(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    println!("[{}] {}", ctx.cache.guild(msg.guild_id.unwrap().0).await.unwrap().name, msg.content.as_str());
    let lock = ctx.data.read().await;
    let db = lock.get::<Database>().unwrap();
    let id = match args.rest().parse::<u64>() {
        Ok(n) => n,
        Err(e) => return Err(CommandError::from(e.to_string())),
    };
    match db.is_banned(id) {
        Ok(b) => msg.reply(ctx, format!("{}", b)).await?,
        Err(e) => return Err(CommandError::from(e.to_string())),
    };
    msg.react(ctx, Unicode("✅".to_string())).await?;
    return Ok(());
}
