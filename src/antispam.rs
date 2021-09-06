use lru_time_cache::LruCache;
use r8limit::RateLimiter;
use serenity::prelude::TypeMapKey;
use std::sync::{Arc, Mutex};
use std::time::Duration;

pub struct AntiSpam {
    pub cache: LruCache<String, RateLimiter>,
}

impl TypeMapKey for AntiSpam {
    type Value = Arc<Mutex<AntiSpam>>;
}

impl AntiSpam {
    pub fn new() -> AntiSpam {
        AntiSpam {
            // TODO: put move the RateLimiter in a struct that also contains a list of all channels they sent message to.
            // TODO: Combine that with a so-so expiration duration (e.g. 120), and if they sent messages to more than 10 channels.
            // TODO: Could also store the message ids so we can delete them once the rate limit has been triggered (or they sent messages in more than 15 channels)
            cache: LruCache::<String, RateLimiter>::with_expiry_duration_and_capacity(Duration::from_secs(600), 10000),
        }
    }

    pub fn check_if_spamming(&mut self, guild_id: u64, user_id: u64) -> bool {
        let key = self.create_key(guild_id, user_id);
        if self.cache.contains_key(&key) {
            return if let Some(limiter) = self.cache.get_mut(&key) {
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
        // There's no cache entry for the user, so we'll create one
        self.cache.insert(key, RateLimiter::new(3, Duration::from_secs(5)));
        false
    }

    fn create_key(&self, guild_id: u64, user_id: u64) -> String {
        format!("{}_{}", guild_id, user_id)
    }
}
