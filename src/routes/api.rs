// Bandurria
//
// Lightweight comment system for static websites
// Copyright: 2025, Valerian Saliou <valerian@valeriansaliou.name>
// License: Mozilla Public License v2.0 (MPL v2.0)

use rocket::http::Status;
use rocket::post;
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

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
    page: String,
    comment: Json<CommentData>,
) -> Result<Json<BaseResponse<()>>, Status> {
    // TODO: store comment in the DB

    Ok(Json(BaseResponse {
        reason: "success",
        data: (),
    }))
}
