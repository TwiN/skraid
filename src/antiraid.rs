use lru_time_cache::LruCache;
use r8limit::RateLimiter;
use serenity::prelude::TypeMapKey;
use std::sync::{Arc, Mutex};
use std::time::Duration;

pub struct AntiRaid {
    pub rate_limiter_cache: LruCache<String, RateLimiter>,
    pub guild_recently_joined_members_cache: LruCache<String, Vec<u64>>,
}

impl TypeMapKey for AntiRaid {
    type Value = Arc<Mutex<AntiRaid>>;
}

impl AntiRaid {
    pub fn new() -> AntiRaid {
        AntiRaid {
            rate_limiter_cache: LruCache::<String, RateLimiter>::with_expiry_duration_and_capacity(Duration::from_secs(600), 1000),
            guild_recently_joined_members_cache: LruCache::<String, Vec<u64>>::with_expiry_duration_and_capacity(Duration::from_secs(30), 1000),
        }
    }

    pub fn check_if_raiding(&mut self, guild_id: u64, user_id: u64) -> bool {
        let key = self.create_key(guild_id);
        if self.rate_limiter_cache.contains_key(&key) {
            self.save_guild_member_id(&key, user_id);
            return if let Some(limiter) = self.rate_limiter_cache.get_mut(&key) {
                if limiter.attempt() {
                    false
                } else {
                    true
                }
            } else {
                println!("FAILED TO RETRIEVE LIMITER FROM CACHE");
                false
            };
        }
        self.save_guild_member_id(&key, user_id);
        // There's no cache entry for the user, so we'll create one
        self.rate_limiter_cache.insert(key, RateLimiter::new(4, Duration::from_secs(10)));
        false
    }

    fn save_guild_member_id(&mut self, key: &str, user_id: u64) {
        if let Some(user_ids) = self.guild_recently_joined_members_cache.get_mut(key) {
            if !user_ids.contains(&user_id) {
                user_ids.push(user_id);
            }
        } else {
            self.guild_recently_joined_members_cache.insert(key.to_string(), vec![user_id]);
        }
    }

    pub fn get_recent_member_ids(&mut self, guild_id: u64) -> Option<&Vec<u64>> {
        return self.guild_recently_joined_members_cache.get(&self.create_key(guild_id));
    }

    pub fn delete_recent_member_ids(&mut self, guild_id: u64) {
        self.guild_recently_joined_members_cache.remove(&self.create_key(guild_id));
    }

    fn create_key(&self, guild_id: u64) -> String {
        format!("{}", guild_id)
    }
}
