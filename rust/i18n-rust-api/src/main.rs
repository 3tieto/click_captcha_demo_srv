#![feature(async_closure)]
#![feature(const_trait_impl)]
#![feature(exact_size_is_empty)]
#![feature(lazy_cell)]
#![feature(impl_trait_in_assoc_type)]
#![feature(type_alias_impl_trait)]
#![feature(let_chains)]
#![allow(non_snake_case)]

mod db;
mod kv;
mod url;

use awp::anypack::FnAny;
use axum::{
  // middleware,
  routing::post,
  // routing::{get, post},
  Router,
};
use trt::TRT;

fn main() -> anyhow::Result<()> {
  // let prepare =
  //   TRT.block_on(async move { xg::PG.force().await.prepare(" INSERT INTO fav.user (uid,cid,rid,ts,aid) VALUES ($1) ON CONFLICT (uid, cid, rid, ts) DO NOTHING RETURNING id").await.unwrap() });

  awp::init();

  let mut router = Router::new();
  macro_rules! post {
        (=> $func:ident) => {
            post!("", $func)
        };
        ($($url:ident),+) => {
            post!($($url=>$url);+)
        };
        ($($url:stmt => $func:ident);+) => {
            $(
                post!(
                    const_str::replace!(
                        const_str::replace!(
                            const_str::unwrap!(const_str::strip_suffix!(stringify!($url), ";")),
                            " ",
                            ""
                        ),
                        "&",
                        ":"
                    ),
                    $func
                );
            )+
        };
        ($url:expr, $func:ident) => {
            router = router.route(
                const_str::concat!('/', $url),
                post(FnAny($crate::url::$func::post)),
            )
        };
    }

  // get!( => stat);
  // post!(li => li;fav=>fav);

  post!(tran, captcha, auth);
  // post!(test);
  // router = router.route("/ws/:li", get(crate::url::ws::get));
  // router = router.route("/ws/:li", post(crate::url::ws::post));
  // post!(hr, q, userFav);

  let default_port = 8850;
  let port = match std::env::var("API_PORT") {
    Ok(val) => val.parse::<u16>().unwrap_or(default_port),
    _ => default_port,
  };

  TRT.block_on(async move {
    // trt::spawn! { update_today().await };
    awp::srv(
      router, //router.layer(middleware::from_fn(client)),
      port,
    )
    .await;
  });
  Ok(())
}
