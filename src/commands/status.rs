use serenity::{
    client::Context,
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
};

#[command]
async fn status(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "I'm fine.").await?;
    Ok(())
}
