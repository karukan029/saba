use alloc::string::String;
use alloc::vec::Vec;
use create::error::Error;

#[derive(Debug,Clone)]
pub struct HttpResponse {
  version: String,
  status_code: u32,
  reason: String,
  headers: Vec<Header>,
  body: String
}

impl HttpResponse {
  pub fn new(raw_response: String) -> Result<Self, Error> {
    // 前後の空白を削除し、改行コードを統一
    let preprocessed = raw_response.trim_start().replace("\r\n", "\n");

    // ステータスラインの分割
  }
}

#[derive(Debug,Clone)]
pub struct Header {
  pub name: String,
  pub value: String,
}

impl Header {
  pub fn new(name: String, value: String) -> Self {
    Self { name, value }
  }
}