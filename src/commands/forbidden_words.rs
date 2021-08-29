use crate::database::Database;
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
#[example("://discorcl.")]
#[aliases(gban)]
#[min_args(1)]
async fn forbid_word(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let word = args.rest();
    {
        let data = ctx.data.read().await;
        let mutex = data.get::<Database>().unwrap();
        let db = mutex.lock().unwrap();
        match db.insert_in_forbidden_words(word.to_string()) {
            Ok(_) => (),
            Err(e) => return Err(CommandError::from(e.to_string())),
        };
    }
    log(ctx, msg, format!("Added '{}' to forbidden words", word));
    return Ok(());
}

#[command]
#[description("Remove a word from the forbidden words")]
#[usage("USER_ID")]
#[example("000000000000000000")]
#[aliases(gunban)]
#[min_args(1)]
async fn unforbid_word(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let word = args.rest();
    {
        let data = ctx.data.read().await;
        let mutex = data.get::<Database>().unwrap();
        let db = mutex.lock().unwrap();
        match db.remove_from_forbidden_words(word.to_string()) {
            Ok(b) => b,
            Err(e) => return Err(CommandError::from(e.to_string())),
        };
    }
    log(ctx, msg, format!("Removed '{}' from forbidden words", word));
    return Ok(());
}

#[command]
#[description("Check if a string contains a forbidden word")]
#[usage("STRING")]
#[example("Claim your free baguette at https://example.com")]
#[min_args(1)]
async fn contains_forbidden_word(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let contains_forbidden_word: bool;
    {
        let data = ctx.data.read().await;
        let mutex = data.get::<Database>().unwrap();
        let db = mutex.lock().unwrap();
        contains_forbidden_word = match db.contains_forbidden_word(args.rest().to_string()) {
            Ok(b) => b,
            Err(e) => return Err(CommandError::from(e.to_string())),
        };
    }
    msg.reply(ctx, format!("{}", contains_forbidden_word)).await?;
    return Ok(());
}

#[command]
#[description("Retrieves a list of all forbidden words")]
async fn get_forbidden_words(ctx: &Context, msg: &Message) -> CommandResult {
    let forbidden_words: Vec<String>;
    {
        let data = ctx.data.read().await;
        let mutex = data.get::<Database>().unwrap();
        let db = mutex.lock().unwrap();
        forbidden_words = match db.get_forbidden_words() {
            Ok(b) => b,
            Err(e) => return Err(CommandError::from(e.to_string())),
        };
    }
    msg.reply(ctx, format!("```{}```", forbidden_words.join("\n"))).await?;
    return Ok(());
}
