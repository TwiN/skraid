use serenity::{
    client::Context,
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
};

#[command("Status")]
#[description("Check the status of the bot")]
#[aliases(status)]
#[bucket(staff)]
async fn status(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "I'm fine.").await?;
    Ok(())
}
