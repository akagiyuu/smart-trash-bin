use anyhow::Result;
use lettre::{
    message::header::ContentType, transport::smtp::authentication::Credentials, AsyncSmtpTransport,
    AsyncTransport, Message, Tokio1Executor,
};

use crate::CONFIG;

pub async fn send_email(subject: String, body: String) -> Result<()> {
    let creds = Credentials::new(
        CONFIG.sender_email.to_owned(),
        CONFIG.sender_password.to_owned(),
    );

    let mailer = AsyncSmtpTransport::<Tokio1Executor>::relay("smtp.gmail.com")?
        .credentials(creds)
        .build();

    let email = Message::builder()
        .from(CONFIG.sender_email.parse().unwrap())
        .to(CONFIG.receiver_email.parse().unwrap())
        .subject(subject)
        .header(ContentType::TEXT_PLAIN)
        .body(body)?;

    mailer.send(email).await?;

    Ok(())
}
