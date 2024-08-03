use std::{
    fs,
    io::{Read, Write},
};

use itertools::Itertools;

use crate::{
    request::Request,
    response::{Body, Response},
};

// TODO: consider configurable transform of input file key
fn get_key_from_path(path: String) -> String {
    path.split("/").skip(2).join("~")
}

pub fn upload(request: Request) -> Response {
    let file_name = get_key_from_path(request.path);
    if let Some(body) = request.body {
        let mut file = fs::OpenOptions::new()
            .create(true)
            .write(true)
            .open(file_name)
            .unwrap();
        file.write_all(&body).unwrap();
    }

    Response {
        code: "200 OK".into(),
        headers: vec![
            "Connection: close".into(),
            "Server: AmazonS3".into(),
            "x-amz-id-2: id2".into(),
        ],
        body: None,
    }
}

pub fn get_object(request: Request) -> Response {
    let file_name = get_key_from_path(request.path);
    let mut file = fs::OpenOptions::new().read(true).open(file_name).unwrap();
    let range: (usize, usize) = request
        .headers
        .get("Range")
        .map(|range_str| {
            range_str
                .replace("bytes=", "")
                .split("-")
                .map(|r| r.parse::<usize>().unwrap())
                .collect_tuple::<(usize, usize)>()
                .unwrap()
        })
        // whole file if no range
        .unwrap_or((0, 22));
    let file_size = range.1 - range.0;
    let mut buffer: Vec<u8> = vec![0; file_size];
    file.read_exact(&mut buffer).unwrap();

    Response {
        code: "200 OK".into(),
        headers: vec![format!("Content-Length: {}", file_size)],
        body: Some(Body::File(buffer)),
    }
}
