// Bandurria
//
// Lightweight comment system for static websites
// Copyright: 2025, Valerian Saliou <valerian@valeriansaliou.name>
// License: Mozilla Public License v2.0 (MPL v2.0)

use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::{get, post};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::helpers::{authentication, mint, notifier, query};
use crate::managers::http::DbConn;

#[derive(Deserialize, Validate)]
pub struct CommentData {
    #[validate(length(equal = 36))]
    comment_id: String,

    #[validate(length(equal = 64))]
    attestation: String,

    #[validate(length(min = 1))]
    name: String,

    #[validate(email)]
    email: String,

    #[validate(length(min = 1))]
    text: String,

    mints: Vec<String>,
    reply_to: Option<String>,
}

#[derive(Serialize)]
pub struct BaseResponse<D> {
    pub reason: &'static str,
    pub data: D,
}

#[derive(Serialize)]
pub struct ChallengeResponseData {
    comment_id: String,
    attestation: String,
    problems: Vec<String>,
    difficulty_expect: mint::MintDifficulty,
    solutions_expect: mint::MintSolutions,
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

    // Read raw input data
    let comment_id = comment.comment_id.as_str();
    let attestation = comment.attestation.as_str();

    // Clean input data
    let email = comment.email.trim();
    let name = comment.name.trim();
    let text = comment.text.trim();

    // Data is empty?
    if email.is_empty() || name.is_empty() || text.is_empty() {
        return Err(Status::BadRequest);
    }

    // Verify attestation
    if !authentication::verify_challenge_attestation(page, comment_id, attestation) {
        return Err(Status::Unauthorized);
    }

    // Verify mints
    let is_mint_verified =
        mint::verify(comment_id, &comment.mints).or(Err(Status::InternalServerError))?;

    if !is_mint_verified {
        return Err(Status::PaymentRequired);
    }

    // Acquire page and author identifiers
    let page_id = query::find_or_create_page_id(&mut db, page).await?;
    let author_id = query::find_or_create_author_id(&mut db, &email, &name).await?;

    // Insert comment for page and author
    query::insert_comment_for_page_id_and_author_id(
        &mut db,
        comment_id,
        &text,
        &page_id,
        &author_id,
        &comment.reply_to,
    )
    .await?;

    // Notify admins of new comment
    notifier::alert_of_new_comment_to_admins(comment_id, page, name, email, text).await;

    Ok(Json(BaseResponse {
        reason: "submitted",
        data: (),
    }))
}

#[post("/challenge?<page>", format = "json")]
pub async fn post_challenge(
    page: &str,
) -> Result<Json<BaseResponse<ChallengeResponseData>>, Status> {
    // Generate a future comment ID and sign it to attest of its origin
    let comment_id = Uuid::new_v4().to_string();
    let attestation = authentication::generate_challenge_attestation(page, &comment_id)?;

    // Generate challenge
    let (problems, difficulty_expect, solutions_expect) =
        mint::challenge(&comment_id).or(Err(Status::InternalServerError))?;

    Ok(Json(BaseResponse {
        reason: "generated",
        data: ChallengeResponseData {
            comment_id,
            attestation,
            problems,
            difficulty_expect,
            solutions_expect,
        },
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
    let comment = query::resolve_comment_status(&mut db, comment_id, "approved").await?;

    if let Some(comment_status) = comment {
        // Process moderation
        if action == "approve" {
            if comment_status == true {
                Ok("Comment has already been approved.")
            } else {
                // Approve comment (mark comment as approved)
                query::update_comment_status(&mut db, comment_id, "approved", true).await?;

                Ok("Comment approved.")
            }
        } else if action == "reject" {
            // Remove comment
            query::remove_comment(&mut db, comment_id).await?;

            Ok("Comment rejected.")
        } else {
            Err(Status::BadRequest)
        }
    } else {
        // Comment does not exist anymore? (treat as non-error)
        Ok("Comment does not exist anymore.")
    }
}
