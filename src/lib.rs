#![deny(clippy::all)]
use napi::bindgen_prelude::*;

#[macro_use]
extern crate napi_derive;

#[napi]
pub fn sum(a: i32, b: i32) -> i32 {
  a + b
}

async fn always_4() -> i32 {
  4
}

#[napi]
pub async fn future_4() -> i32 {
  always_4().await
}

#[napi]
pub struct MyNumber {
  pub num: i32
}

#[napi]
impl MyNumber {
  #[napi]
  pub fn is_4(&self) -> bool {
    self.num == 4
  }

  #[napi(factory)]
  pub fn now_4() -> Self {
    Self { num: 4 }
  }
}