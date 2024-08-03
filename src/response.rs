use itertools::Itertools;

#[derive(Debug)]
pub struct Response {
    pub header: Vec<u8>,
    pub body: Option<Vec<u8>>,
}

pub struct ResponseBuilder {
    status: String,
    headers: Vec<String>,
    body: Option<Vec<u8>>,
}

impl ResponseBuilder {
    pub fn new() -> ResponseBuilder {
        ResponseBuilder {
            status: "200".into(),
            headers: vec![],
            body: None,
        }
    }

    pub fn status(mut self, status: &str) -> Self {
        self.status = status.to_string();
        self
    }

    pub fn header(mut self, header: &str) -> Self {
        self.headers.push(header.to_string());
        self
    }

    pub fn json(mut self, json: String) -> Self {
        self.body = Some(json.as_bytes().to_vec());
        self
    }

    pub fn file(mut self, file: Vec<u8>) -> Self {
        self.body = Some(file);
        self
    }

    pub fn build(self) -> Response {
        let mut headers: Vec<String> = vec![];
        if let Some(body) = &self.body {
            headers.push(format!("Content-Length: {}", body.len()));
        }

        headers.extend(self.headers.clone());

        let response_headers = headers
            .iter()
            .map(|header| header.clone() + "\r\n")
            .join("");
        let header = format!("HTTP/1.1 {0}\r\n{1}\r\n", self.status, response_headers)
            .as_bytes()
            .to_vec();

        Response {
            header,
            body: self.body,
        }
    }
}
