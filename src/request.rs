use std::{collections::HashMap, io::Read, net::TcpStream};

pub type Headers = HashMap<String, String>;
#[derive(Debug)]
pub struct Request {
    pub method: String,
    pub path: String,
    pub headers: Headers,
    pub body: Option<Vec<u8>>,
}

impl Request {
    pub fn new(stream: &mut TcpStream) -> Self {
        let mut buffer: Vec<u8> = vec![];
        let request_without_body: String;

        loop {
            let mut buf: [u8; 64] = [0; 64];
            stream.read(&mut buf).unwrap();
            buffer.append(&mut buf.to_vec());
            let str = String::from_utf8(buffer.clone()).unwrap();
            if str.contains("\r\n\r\n") {
                request_without_body = str;
                break;
            }
        }

        let mut lines: Vec<&str> = request_without_body.split("\r\n").collect();

        let first_el = lines.get(0).cloned().unwrap();
        let first_line_parts = first_el.split(" ").collect::<Vec<&str>>();
        let headers_lines = lines.drain(1..lines.len()).collect::<Vec<&str>>();
        let mut headers: HashMap<String, String> = HashMap::default();

        for header_line in headers_lines {
            let key_value_split = header_line
                .split(":")
                .map(|kv| kv.trim())
                .collect::<Vec<&str>>();
            if key_value_split.len() != 2 {
                continue;
            }
            headers.insert(key_value_split[0].into(), key_value_split[1].into());
        }

        let content_length = headers.get("Content-Length");

        let body = content_length.map(|content_length_str| {
            let content_length: usize = content_length_str.parse().unwrap();
            let mut vv: Vec<u8> = vec![0; content_length];
            stream.read_exact(&mut vv).unwrap();

            vv
        });

        Self {
            method: first_line_parts[0].into(),
            path: first_line_parts[1].into(),
            headers,
            body,
        }
    }
}
