//use lettre::{
//    Email,
//    SmtpClient,
//    ClientSecurity,
//    ClientTlsParameters,
//    smtp::{
//        ConnectionReuseParameters,
//        authentication::{Credentials, Mechanism}
//    }
//};

use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

use native_tls::{Protocol, TlsConnector};

use crate::{models::Confirmation, errors::AuthError, vars};

pub fn send_confirmation_mail(confirmation: &Confirmation) -> Result<(), AuthError> {
    let domain_url = vars::domain_url();
    let expires = confirmation.expires_at.format("%I:%M %p %A, %-d %B, %C%y").to_string();
    let html_text = format!(
        "Please click on the link below to complete registration.<br/>
        <a href=\"{domain}/register?id={id}&email={email}\">Complete registration</a> <br/>
        This link expires on <strong>{expires}</strong>",
        domain=domain_url,
        id=confirmation.id,
        email=confirmation.email,
        expires=expires
    );
    let plain_text = format!(
        "Please visit the link below to complete your registration:\n\
        {domain}/register?id={id}&email={email}\n\
        This link expires on {expires}.",
        domain=domain_url,
        id=confirmation.id,
        email=confirmation.email,
        expires=expires
    );

    let email = Message::builder()
        .from((vars::smtp_sender_name().parse().unwrap()))
        .to((confirmation.email.clone().parse().unwrap()))
        .subject("Complete your registration")
        .body(plain_text)
        .unwrap();

    let creds = Credentials::new(vars::smtp_username(), vars::smtp_password());

    let mailer = SmtpTransport::relay(vars::smtp_host())
        .unwrap()
        .credentials(creds)
        .build();

    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => AuthError::GenericError(String::from("Error sending email: {}", e)),
    }
}