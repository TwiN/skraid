use crate::database::Database;
use crate::utilities::communication::forward_to_maintainer;
use crate::utilities::logging::log;
use serenity::framework::standard::CommandError;
use serenity::model::channel::Message;
use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
};

#[command]
#[description("Add a word to the forbidden words")]
#[usage("WORD")]
#[example("scam-link.com")]
#[aliases(forbid)]
#[min_args(1)]
async fn forbid_word(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let word = args.rest();
    {
        let data = ctx.data.read().await;
        if let Some(mutex) = data.get::<Database>() {
            let db = mutex.lock().unwrap();
            match db.insert_in_forbidden_words(word.to_lowercase()) {
                Ok(_) => (),
                Err(e) => return Err(CommandError::from(e.to_string())),
            };
        }
    }
    log(ctx, msg, format!("Added '{}' to forbidden words", word));
    return Ok(());
}

#[command]
#[description("Remove a word from the forbidden words")]
#[usage("WORD")]
#[example("scam-link.com")]
#[aliases(unforbid)]
#[min_args(1)]
async fn unforbid_word(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let word = args.rest();
    {
        let data = ctx.data.read().await;
        if let Some(mutex) = data.get::<Database>() {
            let db = mutex.lock().unwrap();
            match db.remove_from_forbidden_words(word.to_lowercase()) {
                Ok(b) => b,
                Err(e) => return Err(CommandError::from(e.to_string())),
            };
        }
    }
    log(ctx, msg, format!("Removed '{}' from forbidden words", word));
    return Ok(());
}

#[command]
#[description("Check if a string contains a forbidden word")]
#[usage("STRING")]
#[example("Claim your free baguette at https://scam-link.com")]
#[min_args(1)]
async fn contains_forbidden_word(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let mut contains_forbidden_word: bool = false;
    {
        let data = ctx.data.read().await;
        if let Some(mutex) = data.get::<Database>() {
            let db = mutex.lock().unwrap();
            contains_forbidden_word = match db.contains_forbidden_word(args.rest().to_lowercase()) {
                Ok(b) => b,
                Err(e) => return Err(CommandError::from(e.to_string())),
            };
        }
    }
    msg.reply(ctx, format!("{}", contains_forbidden_word)).await?;
    return Ok(());
}

#[command]
#[description("Retrieves a list of all forbidden words")]
async fn get_forbidden_words(ctx: &Context, msg: &Message) -> CommandResult {
    let mut forbidden_words: Vec<String> = vec![];
    {
        let data = ctx.data.read().await;
        if let Some(mutex) = data.get::<Database>() {
            let db = mutex.lock().unwrap();
            forbidden_words = match db.get_forbidden_words() {
                Ok(b) => b,
                Err(e) => return Err(CommandError::from(e.to_string())),
            };
        }
    }
    msg.reply(ctx, format!("```{}```", forbidden_words.join("\n"))).await?;
    return Ok(());
}

#[command]
#[description("Suggest a word to add to the list of global forbidden words to the maintainer.")]
#[usage("WORD")]
#[example("scam-link.com")]
#[min_args(1)]
#[bucket(suggestion)]
async fn suggest_forbidden_word(ctx: &Context, msg: &Message) -> CommandResult {
    return forward_to_maintainer(ctx, msg).await;
}
