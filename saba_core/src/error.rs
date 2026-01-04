use alloc::string::String;

#[derive(Debug,Clone,PartialEq,Eq)]
pub enum Error {
  NetworkError(String),
  UnexpectedResponse(String),
  InvalidUI(String),
  Other(String),
}