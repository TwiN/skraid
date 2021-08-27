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
async fn global_ban(ctx: &Context, msg: &Message, _: Args) -> CommandResult {
    msg.react(ctx, Unicode("✅".to_string())).await?;
    println!("[{}] {}", ctx.cache.guild(msg.guild_id.unwrap().0).await.unwrap().name, msg.content.as_str());
    return Ok(());
}
