extern crate solicit;
use std::io::{self};
use solicit::http::client::CleartextConnector;
use solicit::client::SimpleClient;

pub trait BasicHttpMethod {
    fn get(&self, path: &str) -> io::Result<()>;
    fn post(&self, path: &str) -> io::Result<()>;
}

pub trait Response {
    fn read(&self);
}

pub trait Request {
    fn set_header(mut &self);
    fn set_method(mut &self);
}

pub trait BasicHttpClient {
    
}

pub struct HttpClient {
    host: String,
    deep: i32,
}