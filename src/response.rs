use itertools::Itertools;

#[derive(Debug)]
pub enum Body {
    Json(String),
    File(Vec<u8>),
}

impl Body {
    pub fn to_string(self) -> String {
        match self {
            Self::Json(json) => json,
            Self::File(bytes) => format!("{:?}", bytes),
        }
    }

    pub fn size(&self) -> usize {
        match self {
            Self::File(bytes) => bytes.len(),
            Self::Json(json) => json.len(),
        }
    }
}

#[derive(Debug)]
pub struct Response {
    pub code: String,
    pub headers: Vec<String>,
    pub body: Option<Body>,
}

impl Response {
    pub fn to_http_response(self) -> String {
        //         "Content-Type: application/x-amz-json-1.0".into(),
        // let body = self.body.

        // TODO: to hashmap
        let mut headers: Vec<String> = vec![];
        headers.extend(self.headers);

        let response_headers = headers
            .iter()
            .map(|header| header.clone() + "\r\n")
            .join("");

        format!(
            "HTTP/1.1 {0}\r\n{1}\r\n{2}",
            self.code,
            response_headers,
            self.body.map(|b| b.to_string()).unwrap_or_default()
        )
    }
}
