use serenity::prelude::TypeMapKey;
use std::collections::HashMap;
use std::env;
use std::sync::{Arc, RwLock};

pub const KEY_PREFIX: &str = "prefix";
pub const KEY_TOKEN: &str = "token";

pub struct Config;

impl TypeMapKey for Config {
    type Value = Arc<RwLock<HashMap<String, String>>>;
}

pub fn load_configuration_map() -> HashMap<String, String> {
    let prefix_from_env = env::var("COMMAND_PREFIX");
    let prefix = prefix_from_env.as_ref().map(String::as_str).unwrap_or("s!");
    let mut map = HashMap::default();
    map.insert(KEY_TOKEN.to_string(), env::var("DISCORD_BOT_TOKEN").expect("token").to_string());
    map.insert(KEY_PREFIX.to_string(), prefix.to_string());
    return map;
}
