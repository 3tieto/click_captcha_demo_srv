use anypack::VecAny;
use tokio::join;

use crate::db::tran;
// use intbin::u64_bin;
// use x0::{fred::interfaces::HashesInterface, R};
// use xg::{Pg, Q};

// Q!(
// li:
//     SELECT cid,rid,ts FROM fav.user LIMIT 2
// );

pub async fn post(body: String) -> awp::any!() {
  let (from_lang, to_lang, htm, txt): (String, String, Vec<String>, Vec<String>) =
    serde_json::from_str(&body)?;

  let (htm, txt) = join!(
    tran::htm(&from_lang, &to_lang, &htm),
    tran::txt(&from_lang, &to_lang, &txt),
  );
  let mut r = VecAny::new();
  r.push(htm?);
  r.push(txt?);
  Ok(r)
}
