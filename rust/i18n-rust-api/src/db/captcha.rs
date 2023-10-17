use anypack::{Pack, VecAny};
use axum::{
  http::{header::HeaderMap, StatusCode},
  response::IntoResponse,
};
use intbin::u64_bin;
use rand::Rng;
use xkv::fred::{interfaces::KeysInterface, prelude::Expiration};

use crate::{db::KV, kv::CAPTCHA};

const JS_MAX_SAFE_INTEGER: u64 = (1u64 << 53) - 1;

pub async fn verify(header: &HeaderMap) -> Result<(), awp::Err> {
  _verify(
    header
      .get("content-type")
      .map(|v| v.to_str().unwrap_or(""))
      .unwrap_or(""),
  )
  .await
}

async fn _verify(body: &str) -> Result<(), awp::Err> {
  let body: Vec<u64> = serde_json::from_str(body)?;
  if body.len() == 7 {
    let key = u64_bin(body[0]);
    let key = [&CAPTCHA[..], &key].concat();
    if let Some(val) = KV.get::<Option<Vec<u8>>, _>(&*key).await? {
      if let Ok(val) = vb::d(val) {
        if click_captcha::verify(&val, &body[1..], 2) {
          return Ok(());
        }
      }
    }
  }

  Err(awp::Err(awp::Error::Response(
    (StatusCode::PRECONDITION_FAILED, new().await?.pack()).into_response(),
  )))
}

pub async fn new() -> anyhow::Result<VecAny> {
  let mut r = VecAny::new();
  let g = click_captcha::gen(780, 780)?;

  let mut flag_li = [0; click_captcha::N * 3];

  for (p, i) in g.1.into_iter().enumerate() {
    r.push(click_captcha::FLAG[i.pos]);
    let p = p * 3;
    flag_li[p] = i.x as _;
    flag_li[p + 1] = i.y as _;
    flag_li[p + 2] = i.size as _;
  }

  let flag_li = vb::e(flag_li);
  let mut key_id = rand::thread_rng().gen_range(0..=JS_MAX_SAFE_INTEGER);
  loop {
    let key = u64_bin(key_id);
    let key = &*[&CAPTCHA[..], &key].concat();
    if !KV.exists::<bool, _>(key).await? {
      KV.set(key, &flag_li[..], Some(Expiration::EX(300)), None, false)
        .await?;
      break;
    }
    key_id = key_id.wrapping_add(1) % JS_MAX_SAFE_INTEGER;
  }
  r.push(g.0);
  r.push(key_id);
  Ok(r)
}
