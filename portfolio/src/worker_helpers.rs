// This module provides Send-safe wrappers around Cloudflare Worker APIs
// Only included when building with SSR feature

use worker::{Env, kv::KvStore, Error};

/// Get KV namespace from environment (Send-safe)
#[worker::send]
pub async fn get_kv(env: &Env, binding: &str) -> Result<KvStore, Error> {
    env.kv(binding)
}

/// Get a value from KV as text (Send-safe)
#[worker::send]
pub async fn kv_get_text(kv: &KvStore, key: &str) -> Result<Option<String>, Error> {
    kv.get(key).text().await.map_err(|e| e.into())
}

/// Put a value into KV (Send-safe)
#[worker::send]
pub async fn kv_put(kv: &KvStore, key: &str, value: String) -> Result<(), Error> {
    kv.put(key, value)?.execute().await.map_err(|e| e.into())
}

/// Get environment variable (Send-safe)
#[worker::send]
pub async fn get_var(env: &Env, name: &str) -> Option<String> {
    env.var(name).ok().map(|v| v.to_string())
}
