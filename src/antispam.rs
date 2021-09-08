use lru_time_cache::LruCache;
use r8limit::{RateLimiter, RefillPolicy};
use serenity::prelude::TypeMapKey;
use std::sync::{Arc, Mutex};
use std::time::Duration;

pub struct AntiSpam {
    pub rate_limit_cache: LruCache<String, RateLimiter>,
    pub guild_member_recent_message_ids_cache: LruCache<String, Vec<ChannelAndMessageId>>,
}

#[derive(Copy, Clone)]
pub struct ChannelAndMessageId {
    pub channel_id: u64,
    pub message_id: u64,
}

impl TypeMapKey for AntiSpam {
    type Value = Arc<Mutex<AntiSpam>>;
}

impl AntiSpam {
    pub fn new() -> AntiSpam {
        AntiSpam {
            rate_limit_cache: LruCache::<String, RateLimiter>::with_expiry_duration_and_capacity(Duration::from_secs(600), 10000),
            guild_member_recent_message_ids_cache: LruCache::<String, Vec<ChannelAndMessageId>>::with_expiry_duration_and_capacity(Duration::from_secs(15), 1000),
        }
    }

    pub fn check_if_spamming(&mut self, guild_id: u64, user_id: u64, channel_id: u64, message_id: u64) -> bool {
        let key = self.create_key(guild_id, user_id);
        if self.rate_limit_cache.contains_key(&key) {
            self.save_message_id_from_guild_member(&key, channel_id, message_id);
            return if let Some(limiter) = self.rate_limit_cache.get_mut(&key) {
                // If the attempt returns false, user is spamming
                // If the attempt returns true, user is not spamming (yet, at least)
                return !limiter.attempt();
            } else {
                println!("FAILED TO RETRIEVE LIMITER FROM CACHE");
                false
            };
        }
        // There's no cache entry for the user, so we'll create one
        self.save_message_id_from_guild_member(&key, channel_id, message_id);
        self.rate_limit_cache.insert(key, RateLimiter::new(4, Duration::from_secs(8)).with_refill_policy(RefillPolicy::Gradual));
        false
    }

    fn save_message_id_from_guild_member(&mut self, key: &str, channel_id: u64, message_id: u64) {
        let channel_and_message_id = ChannelAndMessageId {
            channel_id,
            message_id,
        };
        if let Some(message_ids) = self.guild_member_recent_message_ids_cache.get_mut(key) {
            message_ids.push(channel_and_message_id);
        } else {
            self.guild_member_recent_message_ids_cache.insert(key.to_string(), vec![channel_and_message_id]);
        }
    }

    pub fn get_recent_message_ids(&mut self, guild_id: u64, user_id: u64) -> Option<&Vec<ChannelAndMessageId>> {
        return self.guild_member_recent_message_ids_cache.get(&self.create_key(guild_id, user_id));
    }

    pub fn delete_recent_message_ids(&mut self, guild_id: u64, user_id: u64) {
        self.guild_member_recent_message_ids_cache.remove(&self.create_key(guild_id, user_id));
    }

    fn create_key(&self, guild_id: u64, user_id: u64) -> String {
        format!("{}_{}", guild_id, user_id)
    }
}
