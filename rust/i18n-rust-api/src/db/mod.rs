pub mod captcha;
use xkv::conn;
pub use xkv::fred;

conn!(KV = KV);

pub mod tran;
