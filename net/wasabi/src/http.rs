pub struct HttpClient {}

extern crate alloc;
use alloc::string::String;
use noli::net::lookup_host;
use noli::net::SocketAddr;
use noli::net::TcpStream;
use saba_core::error::Error;
use saba_core::http::HttpResponse;

impl HttpClient {
  pub fn new() -> Self {
    Self {}
  }

  pub fn get(&self, host: String, port: u16, path: String) -> Result<HttpResponse, Error> {
    let ips = match lookup_host(&host) {
      Ok(ips) => ips,
      Err(e) => {
        return Err(Error::NetworkError(format!(
          "Failed to find IP addresses: {:#?}",
          e
        )));
      }
    };

    if ips.len() < 1 {
      return Err(Error::NetworkError(
        "Failed to find IP addresses".to_string(),
      ));
    }

    // intoメソッドは、タプルなどを指定された型に変換するためのメソッド
    // ここでは、(ips[0], port)というタプルをSocketAddr型に変換している
    let socket_addr: SocketAddr = (ips[0], port).into();

    let mut stream = match TcpStream::connect(socket_addr) {
      Ok(stream) => stream,
      Err(_) => {
        return Err(Error::NetworkError(
          "Failed to connect to TCP stream".to_string(),
        ));
      }
    };

    // fromは明示的に型を変換するためのメソッド
    let mut request = String::from("GET /");
    request.push_str(&path);
    request.push_str(" HTTP/1.1\n");

    // ヘッダを追加
    request.push_str("Host: ");
    request.push_str(&host);
    request.push_str("\n");
    request.push_str("Accept: text/html\n");
    request.push_str("Connection: close\n");
    request.push_str("\n");

    // TCPストリームにリクエストを送信
    let _bytes_written = match stream.write(request.as_bytes()){
      Ok(bytes) => bytes,
      Err(_) => {
        return Err(Error::NetworkError(
          "Failed to send a request to TCP stream".to_string(),
        ));
      }
    };

    // TCPストリームからレスポンスを受信
    let mut received = Vec::new();
    loop {
      // u8: 符号なし8ビット整数（1バイト）
      // [0u8; 4096]: 4096バイトのバッファを作成し、すべての要素を0で初期化
      let mut buf = [0u8; 4096];
      let bytes_read = match stream.read(&mut buf) {
        Ok(bytes) => bytes,
        Err(_) => {
          return Err(Error::NetworkError(
            "Failed to receive a request from TCP stream".to_string(),
          ));
        }
      };
      if bytes_read == 0 {
        break;
      }
      received.extend_from_slice(&buf[..bytes_read]);
    }

    // UTF-8のバイト列を文字列に変換して返す
    match core::str::from_utf8(&received) {
      Ok(response) => HttpResponse::new(response.to_string()),
      Err(e) => Err(Error::NetworkError(
        format!("Invalid received response: {}", e)
      )),
    }
  }
}