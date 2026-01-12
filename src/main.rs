#![no_std]
#![no_main]

extern crate alloc;

use crate::alloc::string::ToString;
use net_wasabi::http::HttpClient;
use noli::prelude::*;

fn main() {
    let client = HttpClient::new();
    match client.get("host.test".to_string(), 8000, "test.html".to_string()) {
        Ok(res) => {
            println!("ressponse:\n{:#?}", res);
        }
        Err(e) => {
            println!("Error: {:#?}", e);
        }
    }
}

entry_point!(main);