pub async fn post() -> awp::any!() {
  Ok(crate::db::captcha::new().await?)
}
