use crate::delivery::get_noreply_sender;
use lettre::{
    message::{Mailbox, Message, MultiPart},
    Address,
};

pub fn make_registration_confirm_email(where_to: Address, token: &str) -> Message {
    tracing::warn!("TODO: make_registration_confirm_email needs to have a real template");

    let where_to = Mailbox::new(None, where_to);
    Message::builder()
        .from(get_noreply_sender())
        .to(where_to)
        .subject("Confirm your registration on Oyasumi.app")
        .multipart(MultiPart::alternative_plain_html(
            format!("Registration token: {token}"),
            format!("<h1>Registration</h1><code>{token}</code>"),
        ))
        .unwrap()
}

pub fn make_duplicate_registration_email(where_to: Address) -> Message {
    tracing::warn!("TODO: make_duplicate_registration_email needs to have a real template");

    let where_to = Mailbox::new(None, where_to);
    Message::builder()
        .from(get_noreply_sender())
        .to(where_to)
        .subject("Did you try to register again on Oyasumi.app?")
        .multipart(MultiPart::alternative_plain_html(
            format!("Someone tried to register an account with this email address, but you already have an account. Try resetting your password."),
            format!("<p>Someone tried to register an account with this email address, but you already have an account.</p><p>Try resetting your password.</p>"),
        ))
        .unwrap()
}
