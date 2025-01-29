// Bandurria
//
// Lightweight comment system for static websites
// Copyright: 2025, Valerian Saliou <valerian@valeriansaliou.name>
// License: Mozilla Public License v2.0 (MPL v2.0)

use std::time::Duration;

use reqwest::{redirect, Client};

use crate::APP_CONF;

static HTTP_USER_AGENT: &'static str = concat!(
    env!("CARGO_PKG_NAME"),
    "/",
    env!("CARGO_PKG_VERSION"),
    " (checker)"
);

lazy_static! {
    // Notice: accept invalid certificates, because the root CA chain \
    //   contained in Bandurria might expire if it is not re-compiled, and we \
    //   are dealing with simple proof-of-existence here.
    static ref HTTP_CLIENT: Client = Client::builder()
        .timeout(Duration::from_secs(10))
        .read_timeout(Duration::from_secs(5))
        .connect_timeout(Duration::from_secs(5))
        .pool_idle_timeout(Duration::from_secs(20))
        .pool_max_idle_per_host(1)
        .danger_accept_invalid_certs(true)
        .redirect(redirect::Policy::limited(1))
        .user_agent(HTTP_USER_AGENT)
        .build()
        .unwrap();
}

pub async fn page_url_exists(page_url: &str) -> bool {
    let site_base_url = &APP_CONF.site.site_url;
    let full_uri = format!("{site_base_url}{page_url}");

    debug!("checking that page url exists over http: {full_uri}");

    // Run HTTP HEAD request (since we do not need the response body)
    match HTTP_CLIENT.head(&full_uri).send().await {
        Ok(response) => {
            let status = response.status();

            // Page exists?
            if status.is_success() {
                info!(
                    "page was found to exist over http: {} (code: {})",
                    full_uri,
                    status.as_u16()
                );

                return true;
            }

            // Page does not exist
            warn!(
                "page was found not to exist over http: {} (code: {})",
                full_uri,
                status.as_u16()
            );

            return false;
        }
        Err(err) => {
            error!(
                "could not connect over http to page: {}, because: {}",
                full_uri, err
            )
        }
    }

    // Fallback: page does not exist (in case of any error)
    false
}
