use crate::prelude::*;

#[server(SubmitContactMessage)]
pub async fn submit_contact_message(
  name :    String,
  email :   String,
  subject : String,
  message : String,
  website : String,
) -> Result<(), ServerFnError,> {
  let name = name.trim();
  let email = email.trim();
  let subject = subject.trim();
  let message = message.trim();

  // Honeypot: bots fill this hidden field; silently accept without storing.
  if !website.trim().is_empty() {
    return Ok(());
  }
  if name.is_empty() || name.len() > 120 {
    return Err(ServerFnError::new("Please provide your name."));
  }
  if !email.contains('@') || email.len() > 254 {
    return Err(ServerFnError::new("Please provide a valid email address."));
  }
  if subject.is_empty() || subject.len() > 180 {
    return Err(ServerFnError::new("Please provide a subject."));
  }
  if message.is_empty() || message.len() > 8_000 {
    return Err(ServerFnError::new("Please provide a message under 8,000 characters."));
  }

  let pool = expect_context::<SqlitePool,>();
  sqlx::query(
    "INSERT INTO contact_messages (name, email, subject, message) VALUES (?, ?, ?, ?)",
  )
  .bind(name)
  .bind(email)
  .bind(subject)
  .bind(message)
  .execute(&pool)
  .await
  .map_err(|error| ServerFnError::new(error.to_string()))?;

  Ok(())
}
