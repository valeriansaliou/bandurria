// Bandurria
//
// Lightweight comment system for static websites
// Copyright: 2025, Valerian Saliou <valerian@valeriansaliou.name>
// License: Mozilla Public License v2.0 (MPL v2.0)

use std::time::Duration;

use chrono::Utc;
use reqwest::header::{CONTENT_LENGTH, CONTENT_TYPE};
use reqwest::{redirect, Client, StatusCode};
use rocket::http::Status;

use super::query;
use crate::{managers::http::DbConn, APP_CONF};

pub type AvatarData = Vec<u8>;
pub type AvatarMIME = String;
pub type AvatarBytesSize = u16;
pub type AvatarPixelsSize = u16;

pub static FALLBACK_DATA: &'static [u8] = br##"<svg height="500" viewBox="0 0 500 500" width="500" xmlns="http://www.w3.org/2000/svg"><g fill="none" fill-rule="evenodd"><path d="m0 0h500v500h-500z" fill="#c4c4c4"/><g fill="#fbfbfb"><circle cx="250" cy="204.115842" r="113"/><ellipse cx="250" cy="509.115842" rx="187" ry="215"/></g></g></svg>"##;
pub static FALLBACK_MIME: &'static str = "image/svg+xml";

static GRAVATAR_ENDPOINT: &'static str = "https://gravatar.com";
static IMAGE_MIME_PREFIX: &'static str = "image/";

static HTTP_USER_AGENT: &'static str = concat!(
    env!("CARGO_PKG_NAME"),
    "/",
    env!("CARGO_PKG_VERSION"),
    " (avatar)"
);

const REFRESH_AFTER_SUCCESS: Duration = Duration::from_secs(60 * 60 * 24 * 31); // 1 month
const REFRESH_AFTER_ERRORED: Duration = Duration::from_secs(60 * 60 * 24); // 1 day

#[derive(PartialEq)]
enum CacheStatus {
    Valid,
    Invalid,
    Stale,
    None,
}

#[derive(Clone)]
pub struct Avatar {
    pub data: AvatarData,
    pub mime: AvatarMIME,
    pub size: AvatarBytesSize,
}

pub struct AvatarMaybe {
    pub data: Option<AvatarData>,
    pub mime: Option<AvatarMIME>,
    pub size: AvatarBytesSize,
}

lazy_static! {
    pub static ref AVATAR_SIZE_FULL: AvatarPixelsSize = APP_CONF.avatar.size_pixels * APP_CONF.avatar.scale_factor as AvatarPixelsSize;

    // Notice: accept invalid certificates, because the root CA chain \
    //   contained in Bandurria might expire if it is not re-compiled, and we \
    //   are dealing with avatars here, so there are not much security risks \
    //   except risk of cache pollution attacks.
    static ref HTTP_CLIENT: Client = Client::builder()
        .timeout(Duration::from_secs(10))
        .read_timeout(Duration::from_secs(5))
        .connect_timeout(Duration::from_secs(5))
        .pool_idle_timeout(Duration::from_secs(10))
        .pool_max_idle_per_host(1)
        .danger_accept_invalid_certs(true)
        .redirect(redirect::Policy::none())
        .user_agent(HTTP_USER_AGENT)
        .build()
        .unwrap();
}

async fn pull_cache(db: &mut DbConn, author_id: &str) -> Result<(Option<Avatar>, CacheStatus), ()> {
    debug!("attempting to pull avatar from cache for: {author_id}");

    let avatar_cache = query::resolve_avatar(db, author_id).await.or(Err(()))?;

    if let Some((avatar_maybe, pixels_size, refresh_at)) = avatar_cache {
        // Acquire cached avatar (whether it is valid or stale)
        let avatar = if let (Some(mime), Some(data)) = (avatar_maybe.mime, avatar_maybe.data) {
            Some(Avatar {
                mime,
                data,
                size: avatar_maybe.size,
            })
        } else {
            None
        };

        // Pixels size do not match current configuration?
        if pixels_size != *AVATAR_SIZE_FULL {
            info!(
                "cached avatar pixels size incorrect, marking invalid for: {}",
                author_id
            );

            // Return as 'cache invalid'
            return Ok((None, CacheStatus::Invalid));
        }

        // Refresh date expired? (consider as expired if none)
        if let Some(refresh_at) = refresh_at {
            if Utc::now().naive_utc() >= refresh_at {
                info!(
                    "cached avatar refresh date reached, marking stale for: {}",
                    author_id
                );

                // Return as 'cache stale' (with stale avatar)
                return Ok((avatar, CacheStatus::Stale));
            } else {
                error!(
                    "cached avatar refresh date unknown for: {} (unexpected)",
                    author_id
                );
            }
        }

        info!("cached avatar acquired and still valid for: {author_id}");

        // Return as 'cache valid' (with up-to-date avatar)
        return Ok((avatar, CacheStatus::Valid));
    }

    info!("no cached avatar found for: {author_id}");

    // Return as 'no cache'
    Ok((None, CacheStatus::None))
}

async fn pull_network(email_hash: &str) -> Result<Option<Avatar>, ()> {
    // Normalize email hash into a Gravatar hash
    let gravatar_hash = email_hash.to_lowercase();

    // Generate Gravatar URL
    // Important: we want to HTTP 404 if no Gravatar exists for this email.
    let gravatar_url = format!(
        "{}/avatar/{}?s={}&r=g&d=404",
        GRAVATAR_ENDPOINT, gravatar_hash, *AVATAR_SIZE_FULL
    );

    debug!("pulling avatar from network at: {gravatar_url}");

    match HTTP_CLIENT.get(&gravatar_url).send().await {
        Ok(response) => {
            let status = response.status();

            // Avatar exists?
            if status == StatusCode::OK {
                let mime: AvatarMIME;
                let size: AvatarBytesSize;

                // Check headers first
                let headers = response.headers();

                if let (Some(content_type), Some(content_length)) =
                    (headers.get(CONTENT_TYPE), headers.get(CONTENT_LENGTH))
                {
                    let content_type = content_type.to_str().unwrap_or("");
                    let content_length = content_length
                        .to_str()
                        .unwrap_or("")
                        .parse::<u32>()
                        .unwrap_or(0);

                    if !content_type.starts_with(IMAGE_MIME_PREFIX) {
                        error!(
                            "would pull non-image avatar at: {} (got type: {})",
                            gravatar_url, content_type
                        );

                        // Return error
                        return Err(());
                    }

                    if content_length == 0 || content_length > AvatarBytesSize::MAX as u32 {
                        error!(
                            "would pull empty or over-sized avatar at: {} (got size: {})",
                            gravatar_url, content_length
                        );

                        // Return error
                        return Err(());
                    }

                    // Assign acquired values
                    mime = content_type.to_string();
                    size = content_length as AvatarBytesSize;
                } else {
                    error!(
                        "failed parsing headers for existing avatar at: {}",
                        gravatar_url,
                    );

                    // Return error
                    return Err(());
                }

                // Drain response bytes
                if let Ok(data_bytes) = response.bytes().await {
                    info!("avatar pulled and exists at: {}", gravatar_url);

                    // Assert that sizes match (pulled bytes <> content length)
                    let data_bytes_size = data_bytes.len();

                    if data_bytes_size != size as usize {
                        error!(
                            "got bytes mismatch on pulled avatar at: {} (expected: {}, got: {})",
                            gravatar_url, size, data_bytes_size
                        );

                        // Return error
                        return Err(());
                    }

                    // Avatar found and fetched (success!)
                    return Ok(Some(Avatar {
                        data: data_bytes.to_vec(),
                        mime,
                        size,
                    }));
                }

                error!(
                    "failed pulling bytes for existing avatar at: {}",
                    gravatar_url,
                );

                // Return error
                return Err(());
            }

            // Avatar does not exist?
            if status == StatusCode::NOT_FOUND {
                warn!("avatar pulled and found not to exist at: {}", gravatar_url);

                // No avatar found (but no error)
                return Ok(None);
            }

            // Got unsupported status code
            error!(
                "attempted to pull avatar from network at: {}, but got: {}",
                gravatar_url,
                status.as_u16()
            );

            // Return error
            return Err(());
        }
        Err(err) => {
            error!(
                "could not pull avatar from network at: {}, because: {}",
                gravatar_url, err
            );

            // Return error
            return Err(());
        }
    }
}

pub async fn acquire(db: &mut DbConn, author_id: &str) -> Result<Option<Avatar>, Status> {
    // 1. Acquire cached avatar (if any)
    let (avatar_cache, avatar_cache_status) = pull_cache(db, author_id)
        .await
        .or(Err(Status::InternalServerError))?;

    if avatar_cache_status == CacheStatus::Valid {
        // Short-circuit: return cached avatar
        return Ok(avatar_cache);
    }

    // 2.1. Refresh cached avatar
    // Notice #1: we need to resolve the author email hash first; if the \
    //   author does not exist, then we should 404 ASAP and NOT create a \
    //   database entry.
    // Notice #2: if a pull error occurs, then make sure to keep the \
    //   previously cached avatar in cache (let's not overwrite it!).
    let email_hash = query::resolve_author_email_hash(db, author_id)
        .await?
        .ok_or(Status::NotFound)?;

    let (avatar_network, refresh_after) = if let Ok(avatar) = pull_network(&email_hash).await {
        (avatar, REFRESH_AFTER_SUCCESS)
    } else {
        (avatar_cache, REFRESH_AFTER_ERRORED)
    };

    // 2.2. Store refreshed avatar
    query::insert_or_update_avatar(db, author_id, &avatar_network, refresh_after).await?;

    // Return fetched avatar
    Ok(avatar_network)
}
