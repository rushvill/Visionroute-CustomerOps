//! Opaque session token minting and hashing.

use axum_extra::extract::cookie::{Cookie, SameSite};
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine;
use rand::RngCore;
use sha2::{Digest, Sha256};
use time::Duration as TimeDuration;

use crate::config::{Config, SESSION_COOKIE_NAME};

pub fn mint_session_token() -> String {
    let mut bytes = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut bytes);
    URL_SAFE_NO_PAD.encode(bytes)
}

pub fn hash_session_token(token: &str) -> String {
    let digest = Sha256::digest(token.as_bytes());
    hex::encode(digest)
}

pub fn hash_client_hint(value: &str) -> String {
    let digest = Sha256::digest(value.as_bytes());
    hex::encode(digest)
}

pub struct SessionCookie;

impl SessionCookie {
    pub fn build(token: &str, config: &Config) -> Cookie<'static> {
        let mut cookie = Cookie::new(SESSION_COOKIE_NAME, token.to_owned());
        cookie.set_http_only(true);
        cookie.set_path("/");
        cookie.set_same_site(SameSite::Lax);
        cookie.set_secure(config.cookie_secure());
        cookie.set_max_age(TimeDuration::seconds(config.session_ttl.as_secs() as i64));
        cookie
    }

    pub fn clear(config: &Config) -> Cookie<'static> {
        let mut cookie = Cookie::new(SESSION_COOKIE_NAME, "");
        cookie.set_http_only(true);
        cookie.set_path("/");
        cookie.set_same_site(SameSite::Lax);
        cookie.set_secure(config.cookie_secure());
        cookie.set_max_age(TimeDuration::seconds(0));
        cookie
    }
}
