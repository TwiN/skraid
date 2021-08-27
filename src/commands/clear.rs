use serenity::model::channel::Message;
use serenity::model::channel::ReactionType::Unicode;
use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandError, CommandResult},
};

const MINIMUM_NUMBER_OF_MESSAGES: u64 = 2;
const MAXIMUM_NUMBER_OF_MESSAGES: u64 = 100;

#[command]
#[description = "Clear N messages from a channel"]
#[aliases(clean, nuke)]
#[min_args(1)]
async fn clear(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let number_of_messages = match args.rest().parse::<u64>() {
        Ok(n) => n,
        Err(e) => return Err(CommandError::from(e.to_string())),
    };
    if number_of_messages < MINIMUM_NUMBER_OF_MESSAGES || number_of_messages > MAXIMUM_NUMBER_OF_MESSAGES {
        msg.react(ctx, Unicode("❌".to_string())).await?;
        msg.reply(ctx, format!("Number of messages must be between {} and {} inclusively.", MINIMUM_NUMBER_OF_MESSAGES, MAXIMUM_NUMBER_OF_MESSAGES)).await?;
        return Ok(());
    }
    msg.react(ctx, Unicode("✅".to_string())).await?;
    let message_ids = msg.channel_id.messages(ctx, |retriever| retriever.before(msg.id).limit(number_of_messages)).await?;
    msg.channel_id.delete_messages(ctx, message_ids).await?;
    println!("[{}] {}", msg.guild_id.unwrap(), msg.content.as_str());
    return Ok(());
}
