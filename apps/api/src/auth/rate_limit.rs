//! In-memory login attempt throttle (IP + identifier).

use std::time::{Duration, Instant};

use dashmap::DashMap;

#[derive(Clone)]
pub struct LoginRateLimiter {
    max_attempts: u32,
    window: Duration,
    attempts: DashMap<String, Vec<Instant>>,
}

impl LoginRateLimiter {
    pub fn new(max_attempts: u32, window: Duration) -> Self {
        Self {
            max_attempts,
            window,
            attempts: DashMap::new(),
        }
    }

    pub fn check_allowed(&self, key: &str) -> bool {
        self.prune(key);
        self.attempts
            .get(key)
            .map(|entries| entries.len() < self.max_attempts as usize)
            .unwrap_or(true)
    }

    pub fn record_failure(&self, key: &str) {
        self.prune(key);
        self.attempts
            .entry(key.to_owned())
            .or_default()
            .push(Instant::now());
    }

    pub fn clear(&self, key: &str) {
        self.attempts.remove(key);
    }

    fn prune(&self, key: &str) {
        let cutoff = Instant::now()
            .checked_sub(self.window)
            .unwrap_or_else(Instant::now);
        if let Some(mut entries) = self.attempts.get_mut(key) {
            entries.retain(|instant| *instant >= cutoff);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::LoginRateLimiter;
    use std::time::Duration;

    #[test]
    fn blocks_after_max_failures() {
        let limiter = LoginRateLimiter::new(3, Duration::from_secs(60));
        let key = "test-key";
        assert!(limiter.check_allowed(key));
        limiter.record_failure(key);
        limiter.record_failure(key);
        limiter.record_failure(key);
        assert!(!limiter.check_allowed(key));
        limiter.clear(key);
        assert!(limiter.check_allowed(key));
    }
}
