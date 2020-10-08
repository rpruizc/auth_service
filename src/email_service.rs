extern crate lettre;
extern crate lettre_email;

use lettre::smtp::authentication::IntoCredentials;
use lettre::{SmtpClient, Transport};
use lettre_email::EmailBuilder;

use native_tls::{Protocol, TlsConnector};

use crate::{models::Confirmation, errors::AuthError, vars};

pub fn send_confirmation_mail(confirmation: &Confirmation) -> Result<(), AuthError> {
    let domain_url = vars::domain_url();
    let expires = confirmation.expires_at.format("%I:%M %p %A, %-d %B, %C%y").to_string();

    let html_text = format!(
        "Please click on the link below to complete registration.<br/>
        <a href=\"{domain}/register/{id}\">Complete registration</a> <br/>
        This link expires on <strong>{expires}</strong>",
        domain=domain_url,
        id=confirmation.id,
        expires=expires
    );
    let plain_text = format!(
        "Please visit the link below to complete your registration:\n
        {domain}/register/{id}\n
        This link expires on {expires}.",
        domain=domain_url,
        id=confirmation.id,
        expires=expires
    );

    let email = EmailBuilder::new()
        .from((vars::smtp_sender_name().parse().unwrap()))
        .to((confirmation.email.clone().parse().unwrap()))
        .subject("Complete your registration")
        .text(plain_text)
        .html(html_text)
        .build()
        .unwrap()
        .into();

    let smtp_credentials = (vars::smtp_username(), vars::smtp_password()).into_credentials();

    let mut client = SmtpClient::new_simple(&vars::smtp_host())
        .unwrap()
        .credentials(smtp_credentials)
        .transport();

    let result = client.send(email);

    if result.is_ok() {
        println!("Email sent!");

        Ok(())
    } else {
        println!("Could not send email: {:?}", result);

        Err(AuthError::ProcessError(String::from("Could not send confirmation email")))
    }
}