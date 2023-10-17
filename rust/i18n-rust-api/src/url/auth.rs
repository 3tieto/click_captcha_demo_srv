use axum::http::header::HeaderMap;

pub async fn post(header: HeaderMap, body: String) -> awp::any!() {
  crate::db::captcha::verify(&header).await?;
  let body: (u8, String, String) = serde_json::from_str(&body)?;
  dbg!(body);
  Ok(0)
}
