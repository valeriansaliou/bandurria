// Bandurria
//
// Lightweight comment system for static websites
// Copyright: 2025, Valerian Saliou <valerian@valeriansaliou.name>
// License: Mozilla Public License v2.0 (MPL v2.0)

use rocket::http::Status;
use rocket::post;
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

use crate::helpers::query;
use crate::managers::http::DbConn;

#[derive(Deserialize)]
pub struct CommentData {
    name: String,
    email: String,
    text: String,
    reply_to: Option<String>,
}

#[derive(Serialize)]
pub struct BaseResponse<D> {
    pub reason: &'static str,
    pub data: D,
}

#[post("/comment?<page>", format = "json", data = "<comment>")]
pub async fn post_comment(
    mut db: DbConn,
    page: &str,
    comment: Json<CommentData>,
) -> Result<Json<BaseResponse<()>>, Status> {
    // Clean input data
    let email = comment.email.trim();
    let name = comment.name.trim();
    let text = comment.text.trim();

    // Data is empty?
    if email.is_empty() || name.is_empty() || text.is_empty() {
        return Err(Status::BadRequest);
    }

    // Acquire page and author identifiers
    let page_id = query::find_or_create_page_id(&mut db, page).await?;
    let (author_id, author_trusted) =
        query::find_or_create_author_id(&mut db, &email, &name).await?;

    // Insert comment for page and author
    query::insert_comment_for_page_id_and_author_id(
        &mut db,
        &text,
        &page_id,
        &author_id,
        author_trusted,
        &comment.reply_to,
    )
    .await?;

    Ok(Json(BaseResponse {
        reason: "submitted",
        data: (),
    }))
}
