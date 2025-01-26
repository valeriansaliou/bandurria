// Bandurria
//
// Lightweight comment system for static websites
// Copyright: 2025, Valerian Saliou <valerian@valeriansaliou.name>
// License: Mozilla Public License v2.0 (MPL v2.0)

use super::authentication;
use crate::managers::email as mailer;
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

    let moderation_signature = authentication::sign_payload(comment_id).unwrap_or("".to_string());

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
