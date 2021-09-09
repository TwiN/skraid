use serenity::model::channel::Message;
use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandError, CommandResult},
};

const MINIMUM_NUMBER_OF_MESSAGES: u64 = 2;
const MAXIMUM_NUMBER_OF_MESSAGES: u64 = 100;

#[command("Clear")]
#[description("Clear N messages from the current channel")]
#[aliases(clear, clean, nuke)]
#[usage("NUMBER_OF_MESSAGES_TO_DELETE")]
#[example("20")]
#[num_args(1)]
#[bucket(staff)]
async fn clear(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let number_of_messages = match args.rest().parse::<u64>() {
        Ok(n) => n,
        Err(e) => return Err(CommandError::from(e.to_string())),
    };
    if number_of_messages < MINIMUM_NUMBER_OF_MESSAGES || number_of_messages > MAXIMUM_NUMBER_OF_MESSAGES {
        return Err(CommandError::from(format!("Number of messages must be between {} and {} inclusively.", MINIMUM_NUMBER_OF_MESSAGES, MAXIMUM_NUMBER_OF_MESSAGES)));
    }
    let message_ids = msg.channel_id.messages(ctx, |retriever| retriever.before(msg.id).limit(number_of_messages)).await?;
    msg.channel_id.delete_messages(ctx, message_ids).await?;
    return Ok(());
}
