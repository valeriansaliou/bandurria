// Bandurria
//
// Lightweight comment system for static websites
// Copyright: 2025, Valerian Saliou <valerian@valeriansaliou.name>
// License: Mozilla Public License v2.0 (MPL v2.0)

use std::io::Cursor;

use rocket::get;
use rocket::http::hyper::header::CONTENT_TYPE;
use rocket::http::{Header, Status};
use rocket::request::Request;
use rocket::response::{Responder, Response, Result as ResponseResult};

use crate::helpers::avatar;
use crate::managers::http::DbConn;
use crate::APP_CONF;

pub struct ImageBytes {
    data: Vec<u8>,
    mime: String,
    size: u16,
}

impl<'r, 'o: 'r> Responder<'r, 'o> for ImageBytes {
    fn respond_to(self, _: &'r Request<'_>) -> ResponseResult<'o> {
        Response::build()
            .header(Header::new(CONTENT_TYPE.as_str(), self.mime))
            .sized_body(self.size as usize, Cursor::new(self.data))
            .ok()
    }
}

#[get("/avatar/<author_id>")]
pub async fn get_avatar(mut db: DbConn, author_id: &str) -> Result<ImageBytes, Status> {
    // Ensure avatar service is enabled
    if !APP_CONF.avatar.gravatar {
        return Err(Status::Gone);
    }

    // Acquire actual avatar (or fallback)
    let avatar = avatar::acquire(&mut db, author_id)
        .await?
        .unwrap_or(avatar::Avatar {
            data: avatar::FALLBACK_DATA.to_vec(),
            mime: avatar::FALLBACK_MIME.to_string(),
            size: avatar::FALLBACK_DATA.len() as avatar::AvatarBytesSize,
        });

    Ok(ImageBytes {
        data: avatar.data,
        mime: avatar.mime,
        size: avatar.size,
    })
}
