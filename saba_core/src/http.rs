use alloc::string::String;
use alloc::vec::Vec;
use crate::error::Error; 

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

    let (header, body) = match remaining.split_once("\n\n") {
      Some((h, b)) => {
        let mut headers = Vec::new();
        for header in h.split('\n'){
          let splitted_header: Vec<&str> = header.splitn(2, ':').collect();
          headers.push(Header::new(
            String::from(splitted_header[0].trim()),
            String::from(splitted_header[1].trim())
          ));
        }
        (headers, b)
      },
      None => (Vec::new(), remaining),
    };

    let status_parts: Vec<&str> = status_line.split(' ').collect();

    Ok(Self {
      version: String::from(status_parts[0]),
      status_code: status_parts[1].parse::<u32>().unwrap_or(0),
      reason: String::from(status_parts[2]),
      headers: header,
      body: String::from(body),
    })
  }

  pub fn version(&self) -> String {
    self.version.clone()
  }

  pub fn status_code(&self) -> u32 {
    self.status_code
  }

  pub fn reason(&self) -> String {
    self.reason.clone()
  }

  pub fn headers(&self) -> Vec<Header> {
    self.headers.clone()
  }

  pub fn body(&self) -> String {
    self.body.clone()
  }

  pub fn header_value(&self, name: &str) -> Result<String, String> {
    for h in &self.headers {
      if h.name == name {
        return Ok(h.value.clone());
      }
    }

    Err(format!("faild to find {} in headers", name))
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

#[cfg(test)]
mod tests {
  use alloc::string::ToString;

  use super::*;
  #[test]
  fn test_status_line_only() {
    let raw = "HTTP/1.1 200 OK \n\n".to_string();
    let res = HttpResponse::new(raw).expect("failed to parse http response");
    assert_eq!(res.version(), "HTTP/1.1");
    assert_eq!(res.status_code(), 200);
    assert_eq!(res.reason(), "OK");
  }

  #[test]
  fn test_one_header() {
    let raw = "HTTP/1.1 200 OK \nDate: xx xx xx\n\n".to_string();
    let res = HttpResponse::new(raw).expect("failed to parse http response");
    
    assert_eq!(res.version(), "HTTP/1.1");
    assert_eq!(res.status_code(), 200);
    assert_eq!(res.reason(), "OK");
    assert_eq!(res.header_value("Date"), Ok("xx xx xx".to_string()));
  }

  #[test]
  fn test_two_headers_with_white_space() {
    let raw = "HTTP1/1 200 OK \nDate: xx xx xx \nContent-Length: 42\n\n".to_string();
    let res = HttpResponse::new(raw).expect("failed to parse http response");

    assert_eq!(res.version(), "HTTP1/1");
    assert_eq!(res.status_code(), 200);
    assert_eq!(res.reason(), "OK");

    assert_eq!(res.header_value("Date"), Ok("xx xx xx".to_string()));
    assert_eq!(res.header_value("Content-Length"), Ok("42".to_string()));
  }

  #[test]
  fn test_body() {
    let raw = "HTTP/1.1 200 OK \nDate: xx xx xx\n\nbody message".to_string();
    let res = HttpResponse::new(raw).expect("failed to parse http response");

    assert_eq!(res.version(), "HTTP/1.1");
    assert_eq!(res.status_code(), 200);
    assert_eq!(res.reason(), "OK");

    assert_eq!(res.header_value("Date"), Ok("xx xx xx".to_string()));
    assert_eq!(res.body(), "body message".to_string());
  }

  #[test]
  fn test_invalid() {
    let raw = "HTTP/1.1 200 OK".to_string();
    assert!(HttpResponse::new(raw).is_err());
  }
}