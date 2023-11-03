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
pub async fn maybe_future_4() -> Result<i32> {
  Ok(always_4().await)
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

  #[napi(factory)]
  pub async fn future_4() -> Self {
    let num = future_4().await;
    Self { num }
  }

  #[napi(factory)]
  pub async fn maybe_future_4() -> Result<Self> {
    let async_result = tokio::task::spawn(maybe_future_4()).await;
    // Unwrap JoinError
    let maybe_num = match async_result {
      Ok(num) => num,
      Err(e) => return Err(Error::from_reason(format!("failed to value from future: {e}")))
    };
    match maybe_num {
      Ok(num) => Ok(Self { num }),
      Err(e) => Err(Error::from_reason(format!("maybe_future_4 returned an error: {e}")))
    }
  }
}