// Bandurria
//
// Lightweight comment system for static websites
// Copyright: 2025, Valerian Saliou <valerian@valeriansaliou.name>
// License: Mozilla Public License v2.0 (MPL v2.0)

use rocket::get;
use rocket_dyn_templates::{context, Template};
use serde::Serialize;

#[derive(Serialize)]
pub struct Comment {
    pub name: String,
    pub date: String,
    pub time: String,
    pub text: Vec<String>,
    pub is_owner: bool,
    pub replies: Vec<Comment>,
}

#[get("/comments?<page>")]
pub async fn get_comments(page: String) -> Template {
    // TODO: load comments from the DB

    let comments = vec![
        Comment {
            name: "Valerian".to_string(),
            date: "20/01/2025".to_string(),
            time: "2:58pm".to_string(),
            is_owner: true,

            text: vec![
                "Lorem Ipsum is simply dummy text of the printing and
                    typesetting industry."
                    .to_string(),
                "Lorem Ipsum has been the industry's standard dummy text
                        ever since the 1500s, when an unknown printer took a
                        galley of type and scrambled it to make a type specimen
                        book."
                    .to_string(),
            ],

            replies: vec![],
        },
        Comment {
            name: "Valerian".to_string(),
            date: "20/01/2025".to_string(),
            time: "2:58pm".to_string(),
            is_owner: true,

            text: vec![
                "Lorem Ipsum is simply dummy text of the printing and
                    typesetting industry."
                    .to_string(),
                "Lorem Ipsum has been the industry's standard dummy text
                        ever since the 1500s, when an unknown printer took a
                        galley of type and scrambled it to make a type specimen
                        book."
                    .to_string(),
            ],

            replies: vec![],
        },
    ];

    Template::render("comments", context! { comments })
}
