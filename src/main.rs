use crate::config::{load_configuration_map, Config};
use crate::listeners::handlers::event_handler::Handler;
use commands::clear::*;
use commands::status::*;
use serenity::client::Context;
use serenity::framework::standard::help_commands;
use serenity::framework::standard::macros::{check, group, help};
use serenity::framework::standard::{Args, CommandGroup, CommandOptions, CommandResult, HelpOptions, Reason};
use serenity::framework::StandardFramework;
use serenity::model::channel::Message;
use serenity::model::id::UserId;
use serenity::Client;
use std::collections::HashSet;
use std::sync::{Arc, RwLock};

mod commands;
mod config;
mod listeners;

#[group]
#[commands(status)]
struct General;

#[group]
#[only_in(guilds)]
#[required_permissions(MANAGE_MESSAGES, BAN_MEMBERS)]
#[commands(clear)]
struct Staff;

#[group]
#[checks(Maintainer)]
#[commands(clear)]
struct Maintainer;

#[check]
#[name = "Maintainer"]
async fn maintainer_check(ctx: &Context, msg: &Message, _: &mut Args, _: &CommandOptions) -> Result<(), Reason> {
    if msg.author.id != 7 {
        return Err(Reason::Log("Lacked maintainer permission".to_string()));
    }
    Ok(())
}

#[help]
async fn help(context: &Context, msg: &Message, args: Args, help_options: &'static HelpOptions, groups: &[&'static CommandGroup], owners: HashSet<UserId>) -> CommandResult {
    let _ = help_commands::with_embeds(context, msg, args, help_options, groups, owners).await;
    Ok(())
}

fn create_framework(prefix: String) -> StandardFramework {
    return StandardFramework::new().configure(|c| c.prefix(prefix.as_str()).case_insensitivity(true)).help(&HELP).group(&GENERAL_GROUP).group(&STAFF_GROUP);
}

#[tokio::main]
async fn main() {
    let config = load_configuration_map();
    let mut client = Client::builder(config.get(config::KEY_TOKEN).unwrap().to_string())
        .event_handler(Handler)
        .framework(create_framework(config.get(config::KEY_PREFIX).unwrap().to_string()))
        .await
        .expect("Error creating client");
    {
        let mut data = client.data.write().await;
        data.insert::<Config>(Arc::new(RwLock::new(config)));
    }
    if let Err(why) = client.start().await {
        eprintln!("An error occurred while running the client: {:?}", why);
    }
}
