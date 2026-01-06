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

use alloc::format;

impl HttpResponse {
  pub fn new(raw_response: String) -> Result<Self, Error> {
    // 前後の空白を削除し、改行コードを統一
    let preprocessed_response = raw_response.trim_start().replace("\r\n", "\n");

    // ステータスラインの分割
    let (status_line, remaining) = match preprocessed_response.split_once("\n") {
      Some((line, rem)) => (line, rem),
      None => {
        return Err(Error::NetworkError(format!(
          "invalid http response: {}",
          preprocessed_response
        )));
      }
    };
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