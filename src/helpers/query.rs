// Bandurria
//
// Lightweight comment system for static websites
// Copyright: 2025, Valerian Saliou <valerian@valeriansaliou.name>
// License: Mozilla Public License v2.0 (MPL v2.0)

use std::collections::HashMap;

use rocket::http::Status;
use rocket_db_pools::{sqlx, sqlx::Row};
use serde::Serialize;
use uuid::Uuid;

use super::{checker, normalize, time};
use crate::{managers::http::DbConn, APP_CONF};

#[derive(Serialize)]
pub struct Comment {
    pub id: String,
    pub parent_id: Option<String>,
    pub name: String,
    pub lines: Vec<String>,
    pub datetime: CommentDateTime,
}

#[derive(Serialize)]
pub struct CommentDateTime {
    pub date: String,
    pub time: String,
    pub utc: String,
}

pub async fn find_page_id(db: &mut DbConn, page: &str) -> Result<Option<String>, Status> {
    let page_url = normalize::page_url(page)?;

    let page_id = sqlx::query("SELECT id FROM pages WHERE page = ?")
        .bind(page_url)
        .fetch_optional(&mut ***db)
        .await
        .map_err(|err| {
            error!("Failed loading page: {}, because: {}", page, err);

            Status::InternalServerError
        })?
        .map(|page| page.get("id"));

    Ok(page_id)
}

pub async fn find_or_create_page_id(db: &mut DbConn, page: &str) -> Result<String, Status> {
    let page_url = normalize::page_url(page)?;

    // Safety: assert that page URL is non-empty.
    if page_url.is_empty() {
        return Err(Status::BadRequest);
    }

    match find_page_id(db, page).await? {
        Some(page_id) => Ok(page_id),
        None => {
            // Check that page exists over HTTP first?
            // Notice: this makes sure that attacks cannot fill Bandurria's \
            //   database with random pages, since the page URL will be \
            //   checked over HTTP and must return HTTP 200 (proof of \
            //   existence).
            if APP_CONF.security.check_pages_exist {
                // Actual page do not exist over HTTP, short-circuit here
                if !checker::page_url_exists(&page_url).await {
                    return Err(Status::Gone);
                }
            }

            // Insert new page in the database
            let page_id = Uuid::new_v4().to_string();

            sqlx::query(
                r#"INSERT INTO pages (id, page, created_at)
                    VALUES (?, ?, ?)"#,
            )
            .bind(&page_id)
            .bind(page_url)
            .bind(time::now_datetime_string())
            .execute(&mut ***db)
            .await
            .map_err(|err| {
                error!("Failed creating page: {}, because: {}", page, err);

                Status::InternalServerError
            })?;

            Ok(page_id)
        }
    }
}

pub async fn find_author_id(db: &mut DbConn, email: &str) -> Result<Option<String>, Status> {
    let email_hash = normalize::email_hash(email);

    let author = sqlx::query("SELECT id FROM authors WHERE email_hash = ?")
        .bind(email_hash)
        .fetch_optional(&mut ***db)
        .await
        .map_err(|err| {
            error!("Failed loading author: {}, because: {}", email, err);

            Status::InternalServerError
        })?
        .map(|author| author.get("id"));

    Ok(author)
}

pub async fn find_or_create_author_id(
    db: &mut DbConn,
    email: &str,
    name: &str,
) -> Result<String, Status> {
    match find_author_id(db, email).await? {
        Some(author) => Ok(author),
        None => {
            let author_id = Uuid::new_v4().to_string();
            let email_hash = normalize::email_hash(email);

            sqlx::query(
                r#"INSERT INTO authors (
                    id, email_hash, name, created_at
                )
                    VALUES (?, ?, ?, ?)"#,
            )
            .bind(&author_id)
            .bind(email_hash)
            .bind(name)
            .bind(time::now_datetime_string())
            .execute(&mut ***db)
            .await
            .map_err(|err| {
                error!("Failed creating author: {}, because: {}", email, err);

                Status::InternalServerError
            })?;

            Ok(author_id)
        }
    }
}

pub async fn check_comment_exists(db: &mut DbConn, comment_id: &str) -> Result<bool, Status> {
    let comment_exists = sqlx::query("SELECT id FROM comments WHERE id = ?")
        .bind(comment_id)
        .fetch_optional(&mut ***db)
        .await
        .map_err(|err| {
            error!(
                "Failed checking if comment exists: {}, because: {}",
                comment_id, err
            );

            Status::InternalServerError
        })?
        .is_some();

    Ok(comment_exists)
}

pub async fn resolve_comment_status(
    db: &mut DbConn,
    comment_id: &str,
    status_key: &str,
) -> Result<Option<bool>, Status> {
    let comment_status_value =
        sqlx::query(&format!("SELECT {status_key} FROM comments WHERE id = ?"))
            .bind(comment_id)
            .fetch_optional(&mut ***db)
            .await
            .map_err(|err| {
                error!("Failed resolving comment: {}, because: {}", comment_id, err);

                Status::InternalServerError
            })?
            .map(|comment| comment.get(status_key));

    Ok(comment_status_value)
}

pub async fn update_comment_status(
    db: &mut DbConn,
    comment_id: &str,
    status_key: &str,
    status_value: bool,
) -> Result<(), Status> {
    sqlx::query(&format!(
        "UPDATE comments SET {status_key} = ? WHERE id = ?"
    ))
    .bind(status_value)
    .bind(comment_id)
    .execute(&mut ***db)
    .await
    .map_err(|err| {
        error!(
            "Failed updating comment: {} status: {}, because: {}",
            comment_id, status_key, err
        );

        Status::InternalServerError
    })?;

    Ok(())
}

pub async fn remove_comment(db: &mut DbConn, comment_id: &str) -> Result<(), Status> {
    sqlx::query("DELETE FROM comments WHERE id = ?")
        .bind(comment_id)
        .execute(&mut ***db)
        .await
        .map_err(|err| {
            error!("Failed removing comment: {}, because: {}", comment_id, err);

            Status::InternalServerError
        })?;

    Ok(())
}

pub async fn resolve_comment_page_id(
    db: &mut DbConn,
    comment_id: &str,
) -> Result<Option<String>, Status> {
    let comment_page_id = sqlx::query("SELECT page_id FROM comments WHERE id = ? AND approved = 1")
        .bind(comment_id)
        .fetch_optional(&mut ***db)
        .await
        .map_err(|err| {
            error!("Failed resolving comment: {}, because: {}", comment_id, err);

            Status::InternalServerError
        })?
        .map(|comment| comment.get("page_id"));

    Ok(comment_page_id)
}

pub async fn list_comments_for_page_id(
    db: &mut DbConn,
    page_id: &str,
) -> Result<(Vec<Comment>, HashMap<String, Vec<String>>), Status> {
    let comments: Vec<Comment> = sqlx::query(
        r#"SELECT comments.id, comments.text, comments.created_at,
                comments.reply_to_id, authors.name
            FROM comments INNER JOIN authors ON authors.id = comments.author_id
            WHERE comments.page_id = ? AND comments.approved = 1
            ORDER BY comments.created_at DESC"#,
    )
    .bind(page_id)
    .fetch_all(&mut ***db)
    .await
    .map_err(|err| {
        error!("Failed loading comments: {}", err);

        Status::InternalServerError
    })?
    .into_iter()
    .map(|comment| {
        let (text, created_at): (String, String) = (comment.get("text"), comment.get("created_at"));

        // Parse datetime from string
        let datetime = time::parse_datetime_string(&created_at);

        // Split text into lines
        let text_lines = text
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| line.to_string())
            .collect();

        Comment {
            id: comment.get("id"),
            parent_id: comment.get("reply_to_id"),
            name: comment.get("name"),
            lines: text_lines,
            datetime: CommentDateTime {
                date: time::datetime_to_date_string(&datetime),
                time: time::datetime_to_time_string(&datetime),
                utc: time::datetime_to_utc_string(&datetime),
            },
        }
    })
    .collect();

    // Generate replies references (parent IDs mapping to children IDs)
    let mut replies: HashMap<String, Vec<String>> = HashMap::new();

    for comment in comments.iter() {
        if let Some(parent_id) = comment.parent_id.as_ref() {
            let (parent_id_string, id_string) = (parent_id.to_string(), comment.id.to_string());

            // Insert relationship to replies store
            let parent_store = replies.get_mut(&parent_id_string);

            if let Some(parent_store) = parent_store {
                parent_store.push(id_string);
            } else {
                replies.insert(parent_id_string, vec![id_string]);
            }
        }
    }

    Ok((comments, replies))
}

pub async fn insert_comment_for_page_id_and_author_id(
    db: &mut DbConn,
    comment_id: &str,
    text: &str,
    page_id: &str,
    author_id: &str,
    reply_to_id: &Option<String>,
) -> Result<(), Status> {
    // Security: verify that the replied to comment is on the same page, and \
    //   that replied comment does not loop back to same comment.
    if let Some(reply_to_id) = reply_to_id {
        if reply_to_id == comment_id {
            warn!(
                "Attempted to insert a self-referencing comment reply: {}",
                reply_to_id
            );

            return Err(Status::ImATeapot);
        }

        match resolve_comment_page_id(db, reply_to_id).await? {
            None => {
                warn!(
                    "Attempted to insert comment reply to non-existing comment: {}",
                    reply_to_id
                );

                return Err(Status::Gone);
            }
            Some(reply_page_id) => {
                if reply_page_id != page_id {
                    warn!(
                        "Attempted to insert comment reply on different page: {}",
                        reply_to_id
                    );

                    return Err(Status::NotAcceptable);
                }
            }
        }
    }

    sqlx::query(
        r#"INSERT INTO comments (
            id, text, created_at, author_id, page_id, reply_to_id
        )
            VALUES (?, ?, ?, ?, ?, ?)"#,
    )
    .bind(comment_id)
    .bind(text)
    .bind(time::now_datetime_string())
    .bind(author_id)
    .bind(page_id)
    .bind(reply_to_id)
    .execute(&mut ***db)
    .await
    .map_err(|err| {
        error!("Failed inserting comment: {}", err);

        Status::InternalServerError
    })?;

    Ok(())
}
