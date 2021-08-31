use crate::config::{load_configuration_map, Config};
use crate::database::Database;
use crate::listeners::handlers::event_handler::Handler;
use crate::utilities::logging::log;
use commands::allowlist::*;
use commands::blocklist::*;
use commands::clear::*;
use commands::configure::*;
use commands::forbidden_words::*;
use commands::status::*;
use serenity::client::bridge::gateway::GatewayIntents;
use serenity::client::Context;
use serenity::framework::standard::buckets::LimitedFor;
use serenity::framework::standard::macros::hook;
use serenity::framework::standard::macros::{check, group, help};
use serenity::framework::standard::DispatchError::{NotEnoughArguments, Ratelimited, TooManyArguments};
use serenity::framework::standard::{help_commands, CommandError, DispatchError};
use serenity::framework::standard::{Args, CommandGroup, CommandOptions, CommandResult, HelpOptions, Reason};
use serenity::framework::StandardFramework;
use serenity::model::channel::Message;
use serenity::model::channel::ReactionType::Unicode;
use serenity::model::id::UserId;
use serenity::Client;
use std::collections::HashSet;
use std::sync::{Arc, Mutex, RwLock};

mod commands;
mod config;
mod database;
mod listeners;
mod utilities;

#[group]
#[only_in(guilds)]
#[required_permissions(BAN_MEMBERS)]
#[commands(status, clear)]
struct Utilities;

#[group]
#[only_in(guilds)]
#[required_permissions(BAN_MEMBERS)]
#[commands(get_guild_config, set_alert_channel, set_alert_only)]
struct Configuration;

#[group]
#[only_in(guilds)]
#[required_permissions(BAN_MEMBERS)]
#[commands(suggest_forbidden_word, suggest_blocklist)]
struct Suggestion;

#[group]
#[only_in(guilds)]
#[required_permissions(BAN_MEMBERS)]
#[commands(allowlist, unallowlist, is_allowlisted, get_allowlisted_users)]
struct Allowlist;

#[group]
#[checks(Maintainer)]
#[commands(blocklist, unblocklist, is_blocklisted, forbid_word, unforbid_word, contains_forbidden_word, get_forbidden_words)]
struct Maintainer;

#[check]
#[name(Maintainer)]
async fn maintainer_check(ctx: &Context, msg: &Message, _: &mut Args, _: &CommandOptions) -> Result<(), Reason> {
    let maintainer_id: String;
    {
        let reader = ctx.data.read().await;
        let config = reader.get::<Config>().expect("Expected Config to exist in context data").clone();
        let cfg = config.read().unwrap();
        maintainer_id = cfg.get(config::KEY_MAINTAINER_ID).unwrap().into();
    }
    let maintainer_id_u64 = maintainer_id.parse::<u64>().unwrap();
    if msg.author.id.0 != maintainer_id_u64 {
        return Err(Reason::Log("Lacked maintainer permission".into()));
    }
    Ok(())
}

#[help]
#[individual_command_tip("To get help with an individual command, pass its name as an argument to this command.")]
#[strikethrough_commands_tip_in_guild("")]
#[strikethrough_commands_tip_in_dm("")]
#[lacking_permissions(hide)]
#[lacking_conditions(hide)]
async fn help(ctx: &Context, msg: &Message, args: Args, help_options: &'static HelpOptions, groups: &[&'static CommandGroup], owners: HashSet<UserId>) -> CommandResult {
    let _ = help_commands::with_embeds(ctx, msg, args, help_options, groups, owners).await;
    Ok(())
}

#[hook]
async fn before_hook(ctx: &Context, msg: &Message, _: &str) -> bool {
    log(ctx, msg, msg.content.to_string());
    return true;
}

#[hook]
async fn after_hook(ctx: &Context, msg: &Message, cmd_name: &str, error: Result<(), CommandError>) {
    if let Err(why) = error {
        log(ctx, msg, format!("Error in {}: {}", cmd_name, why));
        let _ = msg.react(ctx, Unicode("❌".into())).await;
        let _ = msg.reply(ctx, format!("Error: `{}`", why)).await;
    } else {
        let _ = msg.react(ctx, Unicode("✅".into())).await;
    }
}

#[hook]
async fn on_dispatch_error(ctx: &Context, msg: &Message, error: DispatchError) {
    match error {
        Ratelimited(_) => {
            let _ = msg.react(ctx, Unicode("⏱".into())).await;
        }
        NotEnoughArguments {
            min,
            given,
        } => {
            let _ = msg.reply(ctx, format!("Need {} arguments, but only got {}.", min, given)).await;
        }
        TooManyArguments {
            max,
            given,
        } => {
            let _ = msg.reply(ctx, format!("Max arguments allowed is {}, but got {}.", max, given)).await;
        }
        _ => (),
    }
}

async fn create_framework(prefix: String) -> StandardFramework {
    return StandardFramework::new()
        .configure(|c| c.prefix(prefix.as_str()).case_insensitivity(true))
        .on_dispatch_error(on_dispatch_error)
        .before(before_hook)
        .after(after_hook)
        .help(&HELP)
        .group(&CONFIGURATION_GROUP)
        .group(&UTILITIES_GROUP)
        .group(&ALLOWLIST_GROUP)
        .group(&SUGGESTION_GROUP)
        .group(&MAINTAINER_GROUP)
        // rate limit after 1 use over 3 seconds
        .bucket("general", |b| b.limit_for(LimitedFor::User).time_span(3).limit(1))
        .await
        // rate limit after 3 uses over 5 seconds
        .bucket("staff", |b| b.limit_for(LimitedFor::Guild).time_span(5).limit(3))
        .await
        // rate limit after 5 uses over 10 seconds
        .bucket("suggestion", |b| b.limit_for(LimitedFor::Guild).time_span(10).limit(5))
        .await;
}

#[tokio::main]
async fn main() {
    let config = load_configuration_map();
    let mut client = Client::builder(config.get(config::KEY_TOKEN).unwrap().to_string())
        .framework(create_framework(config.get(config::KEY_PREFIX).unwrap().to_string()).await)
        .event_handler(Handler)
        .intents(
            GatewayIntents::GUILDS
                | GatewayIntents::GUILD_MEMBERS
                | GatewayIntents::GUILD_BANS
                | GatewayIntents::GUILD_PRESENCES
                | GatewayIntents::GUILD_MESSAGES
                | GatewayIntents::GUILD_MESSAGE_REACTIONS
                | GatewayIntents::DIRECT_MESSAGES
                | GatewayIntents::DIRECT_MESSAGE_REACTIONS,
        ) // Required for #[required_permissions(...)] on #[help]
        .await
        .expect("Error creating client");
    let database_path = config.get(config::KEY_DATABASE_PATH).unwrap().to_string();
    {
        let mut data = client.data.write().await;
        data.insert::<Config>(Arc::new(RwLock::new(config)));
        data.insert::<Database>(Arc::new(Mutex::new(Database::new(database_path))));
    }
    if let Err(why) = client.start().await {
        eprintln!("An error occurred while running the client: {:?}", why);
    }
}
