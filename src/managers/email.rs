// Bandurria
//
// Lightweight comment system for static websites
// Copyright: 2025, Valerian Saliou <valerian@valeriansaliou.name>
// License: Mozilla Public License v2.0 (MPL v2.0)

use std::ops::Deref;
use std::time::Duration;

use lettre::message::Mailbox;
use lettre::transport::smtp::authentication::Credentials;
use lettre::transport::smtp::response::Response as SmtpResponse;
use lettre::transport::smtp::SmtpTransportBuilder;
use lettre::{Message, SmtpTransport, Transport};

use crate::APP_CONF;

const SMTP_TIMEOUT: Duration = Duration::from_secs(6);

lazy_static! {
    static ref SMTP_TRANSPORT: SmtpTransport = make_smtp_transport();
    static ref SMTP_MAILBOX: Mailbox = make_smtp_mailbox();
}

fn make_smtp_transport() -> SmtpTransport {
    let config = &APP_CONF.email.smtp;

    let mut transport: SmtpTransportBuilder;

    // Create appropriate builder
    transport = if config.server_starttls {
        SmtpTransport::starttls_relay(&config.server_host).expect("Add STARTTLS relay")
    } else if config.server_tls {
        SmtpTransport::relay(&config.server_host).expect("Add TLS relay")
    } else {
        panic!("Cannot create SMTP transport, need at least STARTTLS or TLS")
    };

    // Add port and timeout
    transport = transport
        .port(config.server_port)
        .timeout(Some(SMTP_TIMEOUT));

    // Add credentials
    if config.auth_user.is_some() || config.auth_password.is_some() {
        let credentials = Credentials::new(
            config.auth_user.to_owned().unwrap_or("".to_string()),
            config.auth_password.to_owned().unwrap_or("".to_string()),
        );

        transport = transport.credentials(credentials);
    }

    transport.build()
}

fn make_smtp_mailbox() -> Mailbox {
    let config = &APP_CONF.email.identity;

    Mailbox::new(
        Some(config.from_name.to_owned()),
        config
            .from_email
            .parse()
            .expect("Invalid email from address"),
    )
}

pub fn ensure_states() {
    // Ensure all statics are valid (a `deref` is enough to lazily initialize them)
    let (_, _) = (SMTP_TRANSPORT.deref(), SMTP_MAILBOX.deref());
}

pub async fn deliver_faillible(
    to: &str,
    subject: String,
    body: String,
) -> Result<SmtpResponse, ()> {
    let email = Message::builder()
        .from(SMTP_MAILBOX.to_owned())
        .to(to.parse().or(Err(()))?)
        .subject(subject)
        .body(body)
        .or(Err(()))?;

    SMTP_TRANSPORT.send(&email).or(Err(()))
}

pub async fn deliver(to: &str, subject: String, body: String) {
    deliver_faillible(to, subject.to_owned(), body.to_owned())
        .await
        .map_err(|_| {
            error!(
                "failed delivering email to: {}, with subject: '{}'\n\n{}",
                to, &subject, &body
            )
        })
        .map(|_| info!("delivered email to: {}", to))
        .ok();
}
