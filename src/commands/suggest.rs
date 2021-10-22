use crate::utilities::communication::forward_to_maintainer;
use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::channel::Message;

#[command("Suggest")]
#[description("Suggest an improvement to the maintainer")]
#[aliases(suggest)]
#[sub_commands(add_to_word_blocklist, add_to_user_blocklist)]
#[min_args(2)]
async fn suggest(_: &Context, _: &Message) -> CommandResult {
    return Ok(());
}

#[command("AddToWordBlocklist")]
#[description("Suggest a word to add to the word blocklist words to the maintainer.")]
#[aliases(addtowordblocklist)]
#[usage("WORD")]
#[example("scam-link.com")]
#[min_args(1)]
#[bucket(suggestion)]
async fn add_to_word_blocklist(ctx: &Context, msg: &Message) -> CommandResult {
    return forward_to_maintainer(ctx, msg).await;
}

#[command("AddToUserBlocklist")]
#[description("Suggest the addition of a user ID to the global blocklist to the maintainer.")]
#[aliases(addtouserblocklist)]
#[usage("WORD")]
#[example("scam-link.com")]
#[min_args(1)]
#[bucket(suggestion)]
async fn add_to_user_blocklist(ctx: &Context, msg: &Message) -> CommandResult {
    return forward_to_maintainer(ctx, msg).await;
}
