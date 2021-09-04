use crate::config::{Config, KEY_MAINTAINER_ID};
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
#[aliases(forbid)]
#[min_args(1)]
async fn forbid_word(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let word = args.rest();
    {
        let data = ctx.data.read().await;
        let mutex = data.get::<Database>().unwrap();
        let db = mutex.lock().unwrap();
        match db.insert_in_forbidden_words(word.to_lowercase()) {
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
#[aliases(unforbid)]
#[min_args(1)]
async fn unforbid_word(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let word = args.rest();
    {
        let data = ctx.data.read().await;
        let mutex = data.get::<Database>().unwrap();
        let db = mutex.lock().unwrap();
        match db.remove_from_forbidden_words(word.to_lowercase()) {
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
        contains_forbidden_word = match db.contains_forbidden_word(args.rest().to_lowercase()) {
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

#[command]
#[description("Suggest a word to add to the list of global forbidden words to the maintainer.")]
#[usage("WORD")]
#[example("://steamcommrnunity.")]
#[min_args(1)]
#[bucket(suggestion)]
async fn suggest_forbidden_word(ctx: &Context, msg: &Message) -> CommandResult {
    let maintainer_user_id_as_string: String;
    {
        let reader = ctx.data.read().await;
        let config = reader.get::<Config>().expect("Expected Config to exist in context data").clone();
        let cfg = config.read().unwrap();
        maintainer_user_id_as_string = cfg.get(KEY_MAINTAINER_ID).unwrap().clone();
    }
    let maintainer_user_id = match maintainer_user_id_as_string.parse::<u64>() {
        Ok(n) => n,
        Err(e) => return Err(CommandError::from(e.to_string())),
    };
    let user = ctx.http.get_user(maintainer_user_id).await.unwrap();
    let _ = user.direct_message(&ctx, |m| m.content(format!("{} from {}: ```{}```", msg.author.tag(), msg.guild_id.unwrap().0, msg.content))).await;
    return Ok(());
}
