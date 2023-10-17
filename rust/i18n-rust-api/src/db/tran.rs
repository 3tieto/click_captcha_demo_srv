use std::time::Duration;

use lazy_static::lazy_static;
use reqwest::Client;
use reqwest_middleware::ClientBuilder;
use reqwest_retry::{policies::ExponentialBackoff, RetryTransientMiddleware};
use url::form_urlencoded;
use xkv::fred::interfaces::HashesInterface;

use crate::db::KV;

static mut N: usize = 0;
lazy_static! {
  static ref IPV6_PROXY: Vec<String> = std::env::var("IPV6_PROXY")
    .expect("env IPV6_PROXY not defined")
    .split(' ')
    .map(|i| format!("http://{i}"))
    .collect();
}

pub fn proxy() -> &'static str {
  unsafe {
    N = (N + 1) % IPV6_PROXY.len();
    &IPV6_PROXY[N]
  }
}

const URL_TXT: &str = "https://translate.google.com/translate_a/t?client=gtx";

const URL_HTM: &str = const_str::concat!(URL_TXT, "&format=html");

const LIMIT: usize = 2000;

fn split_vec(input: &[impl AsRef<str>], max_length: usize) -> Vec<Vec<String>> {
  let mut result = Vec::new();
  let mut current_vec = Vec::new();
  let mut current_length = 0;

  for s in input {
    let s = s.as_ref();
    let len = s.len();

    if current_length + len > max_length {
      // 当前组的总长度加上下一个字符串长度如果超过最大长度，那么就开始一个新的组。
      result.push(current_vec);
      current_vec = Vec::new();
      current_length = 0;
    }

    current_vec.push(s.to_string());
    current_length += len;
  }

  if !current_vec.is_empty() {
    result.push(current_vec);
  }

  result
}

pub fn vhash(bin: &[u8]) -> Box<[u8]> {
  if bin.len() <= 16 {
    return Box::from(bin);
  }
  xxhash_rust::xxh3::xxh3_128(bin).to_le_bytes().into()
}

pub async fn cached_tran(
  prefix: &str,
  url: &str,
  from_lang: &str,
  to_lang: &str,
  all: &[impl AsRef<str>],
) -> anyhow::Result<Vec<String>> {
  if all.is_empty() {
    return Ok(vec![]);
  }
  let all = all.iter().map(|i| i.as_ref()).collect::<Vec<_>>();

  let hash = all.iter().map(|i| vhash(i.as_bytes())).collect::<Vec<_>>();

  let key = format!("{prefix}:{from_lang}.{to_lang}");
  let exist: Vec<Option<String>> = KV.hmget(&key, hash.clone()).await?;

  let mut to_tran = Vec::with_capacity(all.len());
  let mut to_tran_pos = Vec::with_capacity(all.len());
  let mut r = Vec::with_capacity(all.len());
  for (pos, i) in exist.into_iter().enumerate() {
    r.push(if let Some(i) = i {
      i
    } else {
      to_tran.push(all[pos]);
      to_tran_pos.push(pos);
      "".to_string()
    })
  }

  if !to_tran.is_empty() {
    let traned = tran(url, from_lang, to_lang, &to_tran).await?;
    let mut kv = Vec::with_capacity(traned.len());
    for (i, pos) in traned.into_iter().zip(to_tran_pos) {
      kv.push((hash[pos].clone(), i.clone()));
      r[pos] = i;
    }
    KV.hset(key, kv).await?;
  }
  Ok(r)
}

pub fn lang_map(lang: &str) -> &str {
  match lang {
    "tl" => "fil", // 菲律宾语
    "jw" => "jv",  // 爪哇语
    "iw" => "he",  // 希伯来语
    _ => lang,
  }
}

pub async fn tran(
  url: &str,
  from_lang: &str,
  to_lang: &str,
  all: &[impl AsRef<str>],
) -> anyhow::Result<Vec<String>> {
  if all.is_empty() {
    return Ok(vec![]);
  }
  let from_lang = lang_map(from_lang);
  let to_lang = lang_map(to_lang);

  let client = ClientBuilder::new(
    Client::builder()
      .proxy(reqwest::Proxy::https(proxy())?)
      .build()?,
  )
  .with(RetryTransientMiddleware::new_with_policy(
    ExponentialBackoff::builder()
      .retry_bounds(Duration::from_millis(1), Duration::from_secs(1))
      .build_with_max_retries(9),
  ))
  .build();

  let all = split_vec(all, LIMIT);
  let mut r = Vec::with_capacity(all.len());
  for li in all {
    let form = li
      .iter()
      .map(|i| {
        format!(
          "q={}",
          form_urlencoded::byte_serialize(i.as_bytes()).collect::<String>()
        )
      })
      .collect::<Vec<_>>()
      .join("&");

    let res = client
      .post(format!("{url}&tl={to_lang}&sl={from_lang}"))
      .header("Content-Type", "application/x-www-form-urlencoded")
      .body(form)
      .send()
      .await?
      .text()
      .await?;
    // let res = Retry::spawn(ExponentialBackoff::from_millis(10).take(9), async || {
    //   Ok::<_, reqwest::Error>((&req).clone().send().await?.text().await?)
    // })
    // .await?;
    let res: Vec<String> = serde_json::from_str(&res)?;
    r.extend(res);
  }
  Ok(r)
}

pub async fn txt(
  from_lang: &str,
  to_lang: &str,
  all: &[impl AsRef<str>],
) -> anyhow::Result<Vec<String>> {
  cached_tran("txt", URL_TXT, from_lang, to_lang, all).await
}

pub async fn htm(
  from_lang: &str,
  to_lang: &str,
  all: &[impl AsRef<str>],
) -> anyhow::Result<Vec<String>> {
  cached_tran("htm", URL_HTM, from_lang, to_lang, all).await
}
