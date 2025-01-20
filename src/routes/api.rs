// Bandurria
//
// Lightweight comment system for static websites
// Copyright: 2025, Valerian Saliou <valerian@valeriansaliou.name>
// License: Mozilla Public License v2.0 (MPL v2.0)

use rocket::get;
use rocket::http::Status;
use rocket::serde::json::Json;
use serde::Serialize;

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
