use crate::database::Database;
use crate::utilities::logging::log;
use serenity::framework::standard::CommandError;
use serenity::model::channel::Message;
use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
};

#[command("UserBlocklist")]
#[description("Interact with the global user blocklist")]
#[aliases(userblocklist, ubl)]
#[sub_commands(user_blocklist_add, user_blocklist_remove, user_blocklist_search)]
#[min_args(1)]
async fn user_blocklist(_: &Context, _: &Message) -> CommandResult {
    Ok(())
}

#[command("add")]
#[description("Add user ID to the global user blocklist")]
#[aliases(add)]
#[usage("USER_ID [REASON]")]
#[example("000000000000000000 Scam")]
#[min_args(1)]
async fn user_blocklist_add(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let ids = args.single::<String>().unwrap();
    let ids_iterator = ids.split(",");
    let reason = args.rest();
    {
        let data = ctx.data.read().await;
        if let Some(mutex) = data.get::<Database>() {
            let db = mutex.lock().unwrap();
            for id in ids_iterator {
                let id = match id.parse::<u64>() {
                    Ok(n) => n,
                    Err(e) => return Err(CommandError::from(e.to_string())),
                };
                match db.insert_in_user_blocklist(id, reason.to_string()) {
                    Ok(_) => (),
                    Err(e) => return Err(CommandError::from(e.to_string())),
                };
                log(ctx, msg, format!("Successfully added id={} to blocklist for reason={}", id, reason));
            }
        }
    }
    return Ok(());
}

#[command("remove")]
#[description("Remove user ID from the global user blocklist")]
#[aliases(delete)]
#[usage("USER_ID")]
#[example("000000000000000000")]
#[num_args(1)]
async fn user_blocklist_remove(ctx: &Context, _: &Message, args: Args) -> CommandResult {
    let id = match args.rest().parse::<u64>() {
        Ok(n) => n,
        Err(e) => return Err(CommandError::from(e.to_string())),
    };
    {
        let data = ctx.data.read().await;
        if let Some(mutex) = data.get::<Database>() {
            let db = mutex.lock().unwrap();
            match db.remove_from_user_blocklist(id) {
                Ok(b) => b,
                Err(e) => return Err(CommandError::from(e.to_string())),
            };
        }
    }
    return Ok(());
}

#[command("search")]
#[description("Check if a user ID is in the global user blocklist")]
#[aliases(check, test)]
#[usage("USER_ID")]
#[example("000000000000000000")]
#[num_args(1)]
async fn user_blocklist_search(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let id = match args.rest().parse::<u64>() {
        Ok(n) => n,
        Err(e) => return Err(CommandError::from(e.to_string())),
    };
    let mut is_banned: bool = false;
    {
        let data = ctx.data.read().await;
        if let Some(mutex) = data.get::<Database>() {
            let db = mutex.lock().unwrap();
            is_banned = match db.is_in_user_blocklist(id) {
                Ok(b) => b,
                Err(e) => return Err(CommandError::from(e.to_string())),
            };
        }
    }
    msg.reply(ctx, format!("{}", is_banned)).await?;
    return Ok(());
}
