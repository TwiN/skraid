use crate::config::{load_configuration_map, Config};
use crate::listeners::handlers::event_handler::Handler;
use commands::clear::*;
use commands::global_ban::*;
use commands::status::*;
use serenity::client::bridge::gateway::GatewayIntents;
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
#[required_permissions(BAN_MEMBERS)]
#[commands(clear)]
struct Staff;

#[group]
#[checks(Maintainer)]
#[commands(global_ban)]
struct Maintainer;

#[check]
#[name(Maintainer)]
async fn maintainer_check(ctx: &Context, msg: &Message, _: &mut Args, _: &CommandOptions) -> Result<(), Reason> {
    let maintainer_id: String;
    {
        let reader = ctx.data.read().await;
        let config = reader.get::<Config>().expect("Expected Config to exist in context data").clone();
        let cfg = config.read().unwrap();
        maintainer_id = cfg.get(config::KEY_MAINTAINER_ID).unwrap().to_string();
    }
    let maintainer_id_u64 = maintainer_id.parse::<u64>().unwrap();
    if msg.author.id.0 != maintainer_id_u64 {
        return Err(Reason::Log("Lacked maintainer permission".to_string()));
    }
    Ok(())
}

#[help]
#[strikethrough_commands_tip_in_guild("")]
#[lacking_permissions(hide)]
#[lacking_conditions(hide)]
async fn help(ctx: &Context, msg: &Message, args: Args, help_options: &'static HelpOptions, groups: &[&'static CommandGroup], owners: HashSet<UserId>) -> CommandResult {
    let _ = help_commands::with_embeds(ctx, msg, args, help_options, groups, owners).await;
    Ok(())
}

fn create_framework(prefix: String) -> StandardFramework {
    return StandardFramework::new()
        .configure(|c| c.prefix(prefix.as_str()).case_insensitivity(true))
        .help(&HELP)
        .group(&GENERAL_GROUP)
        .group(&STAFF_GROUP)
        .group(&MAINTAINER_GROUP);
}

#[tokio::main]
async fn main() {
    let config = load_configuration_map();
    let mut client = Client::builder(config.get(config::KEY_TOKEN).unwrap().to_string())
        .framework(create_framework(config.get(config::KEY_PREFIX).unwrap().to_string()))
        .event_handler(Handler)
        .intents(GatewayIntents::all()) // Required for #[required_permissions(...)] on #[help]
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
