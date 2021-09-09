use crate::database::Database;
use crate::utilities::logging::log;
use serenity::framework::standard::CommandError;
use serenity::model::channel::Message;
use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
};

#[command("WordBlocklist")]
#[description("Interact with the word blocklist")]
#[aliases(wordblocklist, wbl)]
#[sub_commands(word_blocklist_add, word_blocklist_remove, word_blocklist_list, word_blocklist_test)]
#[min_args(1)]
async fn word_blocklist(_: &Context, _: &Message) -> CommandResult {
    return Ok(());
}

#[command("add")]
#[description("Add a word to the word blocklist")]
#[usage("WORD")]
#[example("scam-link.com")]
#[min_args(1)]
async fn word_blocklist_add(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let word = args.rest();
    {
        let data = ctx.data.read().await;
        if let Some(mutex) = data.get::<Database>() {
            let db = mutex.lock().unwrap();
            match db.insert_in_word_blocklist(word.to_lowercase()) {
                Ok(_) => (),
                Err(e) => return Err(CommandError::from(e.to_string())),
            };
        }
    }
    log(ctx, msg, format!("Added '{}' to forbidden words", word));
    return Ok(());
}

#[command("remove")]
#[description("Remove a word from the forbidden words")]
#[aliases(delete)]
#[usage("WORD")]
#[example("scam-link.com")]
#[min_args(1)]
async fn word_blocklist_remove(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let word = args.rest();
    {
        let data = ctx.data.read().await;
        if let Some(mutex) = data.get::<Database>() {
            let db = mutex.lock().unwrap();
            match db.remove_from_word_blocklist(word.to_lowercase()) {
                Ok(b) => b,
                Err(e) => return Err(CommandError::from(e.to_string())),
            };
        }
    }
    log(ctx, msg, format!("Removed '{}' from forbidden words", word));
    return Ok(());
}

#[command("list")]
#[description("Retrieves a list of all blocklisted words")]
#[aliases(get)]
async fn word_blocklist_list(ctx: &Context, msg: &Message) -> CommandResult {
    let mut forbidden_words: Vec<String> = vec![];
    {
        let data = ctx.data.read().await;
        if let Some(mutex) = data.get::<Database>() {
            let db = mutex.lock().unwrap();
            forbidden_words = match db.get_words_in_word_blocklist() {
                Ok(b) => b,
                Err(e) => return Err(CommandError::from(e.to_string())),
            };
        }
    }
    msg.reply(ctx, format!("```{}```", forbidden_words.join("\n"))).await?;
    return Ok(());
}

#[command("test")]
#[description("Check if a string contains a word in the word blocklist")]
#[aliases(check)]
#[usage("STRING")]
#[example("Claim your free baguette at https://scam-link.com")]
#[min_args(1)]
async fn word_blocklist_test(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let mut contains: bool = false;
    {
        let data = ctx.data.read().await;
        if let Some(mutex) = data.get::<Database>() {
            let db = mutex.lock().unwrap();
            contains = match db.contains_word_in_word_blocklist(args.rest().to_lowercase()) {
                Ok(b) => b,
                Err(e) => return Err(CommandError::from(e.to_string())),
            };
        }
    }
    msg.reply(ctx, format!("{}", contains)).await?;
    return Ok(());
}
