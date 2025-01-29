// Bandurria
//
// Lightweight comment system for static websites
// Copyright: 2025, Valerian Saliou <valerian@valeriansaliou.name>
// License: Mozilla Public License v2.0 (MPL v2.0)

use std::collections::HashMap;

use rocket::get;
use rocket::http::Status;
use rocket_dyn_templates::{context, Template};

use crate::config::config::ConfigI18N;
use crate::helpers::query;
use crate::managers::http::DbConn;
use crate::APP_CONF;

#[derive(Serialize)]
struct CommentsOptions<'a> {
    i18n: &'a ConfigI18N,
    avatar: Option<CommentsOptionsAvatar>,
}

#[derive(Serialize)]
struct CommentsOptionsAvatar {
    avatar_endpoint: String,
    size_image: u16,
}

lazy_static! {
    static ref COMMENTS_OPTIONS: CommentsOptions<'static> = CommentsOptions {
        i18n: &APP_CONF.i18n,
        avatar: if APP_CONF.avatar.gravatar {
            Some(CommentsOptionsAvatar {
                avatar_endpoint: format!("{}/image/avatar", APP_CONF.site.comments_url),
                size_image: APP_CONF.avatar.size_pixels,
            })
        } else {
            None
        }
    };
}

#[get("/comments?<page>")]
pub async fn get_comments(mut db: DbConn, page: &str) -> Result<Template, Status> {
    // Fetch all comments for page
    let (comments, replies) = match query::find_page_id(&mut db, page).await? {
        Some(page_id) => query::list_comments_for_page_id(&mut db, &page_id).await?,
        None => (Vec::new(), HashMap::new()),
    };

    Ok(Template::render(
        "bandurria",
        context! { comments, replies, ctx: &*COMMENTS_OPTIONS },
    ))
}
