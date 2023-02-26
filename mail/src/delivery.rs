use lettre::transport::smtp::authentication::Credentials;
use lettre::{message::Mailbox, Message};
use lettre::{AsyncSmtpTransport, AsyncTransport};

pub fn get_noreply_sender() -> Mailbox {
    Mailbox::new(
        Some("Oyasumi".to_string()),
        std::env!("MAIL_NOREPLY_ACCOUNT")
            .parse()
            .expect("Email address provided at compile time is wrong!!"),
    )
}

pub async fn send_message(message: Message) -> Result<(), lettre::transport::smtp::Error> {
    let transport =
        AsyncSmtpTransport::<lettre::Tokio1Executor>::relay(std::env!("MAIL_SERVER_HOST"))?
            .credentials(Credentials::new(
                std::env!("MAIL_NOREPLY_ACCOUNT").to_string(),
                std::env!("MAIL_NOREPLY_PASSWORD").to_string(),
            ))
            .build();
    transport.send(message).await?;
    Ok(())
}
