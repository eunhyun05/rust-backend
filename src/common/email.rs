use resend_rs::types::CreateEmailBaseOptions;
use resend_rs::{Resend, Result};
use crate::config::CONFIG;

#[allow(dead_code)]
pub async fn send_email(to: Vec<String>, subject: &str, body: &str) -> Result<()> {
    let resend = Resend::new(&*CONFIG.resend_api_key);

    let from = "Acme <onboarding@resend.dev>";

    let email = CreateEmailBaseOptions::new(from, to, subject)
        .with_html(body);

    let _email = resend.emails.send(email).await?;

    Ok(())
}