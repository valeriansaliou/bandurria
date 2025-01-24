// Bandurria
//
// Lightweight comment system for static websites
// Copyright: 2025, Valerian Saliou <valerian@valeriansaliou.name>
// License: Mozilla Public License v2.0 (MPL v2.0)

use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::{get, post};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::helpers::{authentication, notifier, query};
use crate::managers::http::DbConn;

#[derive(Deserialize, Validate)]
pub struct CommentData {
    #[validate(length(min = 1))]
    name: String,

    #[validate(email)]
    email: String,

    #[validate(length(min = 1))]
    text: String,

    reply_to: Option<String>,
}

#[derive(Serialize)]
pub struct BaseResponse<D> {
    pub reason: &'static str,
    pub data: D,
}

#[get("/")]
pub async fn get_base() -> Result<Json<BaseResponse<()>>, Status> {
    Ok(Json(BaseResponse {
        reason: "welcome",
        data: (),
    }))
}

#[post("/comment?<page>", format = "json", data = "<comment>")]
pub async fn post_comment(
    mut db: DbConn,
    page: &str,
    comment: Json<CommentData>,
) -> Result<Json<BaseResponse<()>>, Status> {
    // Data is invalid?
    if comment.validate().is_err() {
        return Err(Status::UnprocessableEntity);
    }

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
    let comment_id = query::insert_comment_for_page_id_and_author_id(
        &mut db,
        &text,
        &page_id,
        &author_id,
        author_trusted,
        &comment.reply_to,
    )
    .await?;

    // Notify admins of new comment
    notifier::alert_of_new_comment_to_admins(&comment_id, page, name, email, text, author_trusted)
        .await;

    Ok(Json(BaseResponse {
        reason: "submitted",
        data: (),
    }))
}

#[get("/admin/moderate/<comment_id>?<signature>&<action>")]
pub async fn get_admin_moderate_comment(
    mut db: DbConn,
    comment_id: &str,
    signature: &str,
    action: &str,
) -> Result<&'static str, Status> {
    // Important: verify signature first things first
    if !authentication::verify_payload(comment_id, signature) {
        return Err(Status::Unauthorized);
    }

    // Resolve comment
    let comment =
        query::resolve_comment_status_and_author_id(&mut db, comment_id, "approved").await?;

    if let Some((comment_status, comment_author_id)) = comment {
        // Process moderation
        if action == "approve" {
            if comment_status == true {
                Ok("Comment has already been approved.")
            } else {
                // Approve comment (mark comment as verified)
                query::update_comment_status(&mut db, comment_id, "approved", true).await?;
                query::update_comment_status(&mut db, comment_id, "verified", true).await?;

                // Mark user as trusted (if not already trusted)
                query::update_author_trusted(&mut db, &comment_author_id, true).await?;

                Ok("Comment approved.")
            }
        } else if action == "reject" {
            // Remove comment
            query::remove_comment(&mut db, comment_id).await?;

            // Reset user trust marker (marking the user as not trusted)
            query::update_author_trusted(&mut db, &comment_author_id, false).await?;

            Ok("Comment rejected.")
        } else {
            Err(Status::BadRequest)
        }
    } else {
        // Comment does not exist anymore? (treat as non-error)
        Ok("Comment does not exist anymore.")
    }
}
