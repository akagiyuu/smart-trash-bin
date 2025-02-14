use anyhow::Result;
use lettre::{
    message::header::ContentType, transport::smtp::authentication::Credentials, Message,
    SmtpTransport, Transport,
};

use crate::CONFIG;

pub fn send_email(subject: String, body: String) -> Result<()> {
    let creds = Credentials::new(
        CONFIG.sender_email.to_owned(),
        CONFIG.sender_password.to_owned(),
    );

    let mailer = SmtpTransport::relay("smtp.gmail.com")
        .unwrap()
        .credentials(creds)
        .build();

    let email = Message::builder()
        .from(CONFIG.sender_email.parse().unwrap())
        .to(CONFIG.receiver_email.parse().unwrap())
        .subject(subject)
        .header(ContentType::TEXT_PLAIN)
        .body(body)
        .unwrap();

    mailer.send(&email)?;

    Ok(())
}
