// Bandurria
//
// Lightweight comment system for static websites
// Copyright: 2025, Valerian Saliou <valerian@valeriansaliou.name>
// License: Mozilla Public License v2.0 (MPL v2.0)

use super::{authentication, query};
use crate::managers::email as mailer;
use crate::managers::http::DbConn;
use crate::APP_CONF;

pub async fn alert_of_new_comment_to_admins(
    comment_id: &str,
    page: &str,
    name: &str,
    email: &str,
    text: &str,
) {
    let site_url = &APP_CONF.site.site_url;

    let moderation_url = format!(
        "{}/api/admin/moderate/{}/",
        APP_CONF.site.comments_url, comment_id
    );

    let moderation_signature =
        authentication::generate_admin_comment_signature("moderate", comment_id)
            .unwrap_or("".to_string());

    // Generate moderation links
    let moderation_links = format!(
        r#"You can approve this comment:

✅ {moderation_url}?signature={moderation_signature}&action=approve

Or reject it (this will remove the comment):

❌ {moderation_url}?signature={moderation_signature}&action=reject"#
    );

    // Generate email contents
    let email_subject = format!("💬 New comment on {}", APP_CONF.site.name);

    let email_body = format!(
        r#"{name} ({email}) said:

{text}

{site_url}{page}#comment-{comment_id}

—

{moderation_links}"#
    );

    // Send emails to all admins
    for admin_email in APP_CONF.site.admin_emails.iter() {
        mailer::deliver(admin_email, email_subject.to_owned(), email_body.to_owned()).await
    }
}

pub async fn alert_of_reply_comment_from_admin_if_needed(
    db: &mut DbConn,
    parent_comment_id: &str,
    reply_comment_id: &str,
) {
    // 1. Resolve parent comment author email (if they opted-in to email alerts)
    let parent_author_result =
        query::resolve_comment_author_email_name(db, parent_comment_id).await;

    // 2. Resolve reply comment author email hash (ensure reply comment author \
    //   is an administrator)
    let reply_author_result = query::resolve_comment_author_email_name(db, reply_comment_id).await;

    // 3. Resolve reply comment page and text
    let reply_page_and_text_result =
        query::resolve_comment_page_and_text(db, reply_comment_id).await;

    match (
        parent_author_result,
        reply_author_result,
        reply_page_and_text_result,
    ) {
        (Ok(Some(parent_author)), Ok(Some(reply_author)), Ok(Some(reply_page_and_text))) => {
            let (parent_email, reply_email_hash, reply_name) =
                (parent_author.1, reply_author.0, reply_author.2);

            let (reply_page, reply_text) = (reply_page_and_text.0, reply_page_and_text.1);

            debug!(
                "checking if should alert of reply to email: {:?}, replier: {} ({})",
                parent_email, reply_email_hash, reply_name
            );

            // Parent comment has an email set?
            if let Some(parent_email) = parent_email {
                // Reply author is administrator? (we only want to notify of \
                //   replies from administrators)
                if authentication::check_email_hash_is_admin(&reply_email_hash) {
                    info!(
                        "will alert of reply comment to: {} (from admin and opted-in)",
                        parent_comment_id
                    );

                    // Deliver reply notification email
                    alert_of_reply_comment_from_admin(
                        reply_comment_id,
                        &parent_email,
                        &reply_page,
                        &reply_name,
                        &reply_text,
                    )
                    .await;
                } else {
                    debug!(
                        "not alerting of reply comment to: {} (not from admin)",
                        parent_comment_id
                    )
                }
            } else {
                debug!(
                    "not alerting of reply comment to: {} (did not opt-in)",
                    parent_comment_id
                )
            }
        }
        _ => {
            error!(
                "error alerting of reply comment for comment chain: {} -> {}",
                reply_comment_id, parent_comment_id
            )
        }
    }
}

async fn alert_of_reply_comment_from_admin(
    reply_comment_id: &str,
    parent_email: &str,
    page: &str,
    reply_name: &str,
    reply_text: &str,
) {
    let site_url = &APP_CONF.site.site_url;

    // Generate email contents
    let email_subject = format!("↪️ New reply on {}", APP_CONF.site.name);

    let email_body = format!(
        r#"{reply_name} replied to your comment and said:

{reply_text}

{site_url}{page}#comment-{reply_comment_id}"#
    );

    // Send email to parent comment author
    mailer::deliver(
        parent_email,
        email_subject.to_owned(),
        email_body.to_owned(),
    )
    .await
}
