use std::{
    fs,
    io::{Read, Seek},
};

use crate::{
    request::Request,
    response::{Response, ResponseBuilder},
};

// TODO: to config
const FILE_NAME: &str = "file.mp3";

pub fn upload(_request: Request) -> Response {
    let response_builder = ResponseBuilder::new();
    let response = response_builder
        .status("200 OK")
        .header("Connection: close")
        .header("Server: Amazon S3")
        .header("x-amz-id-2: id2")
        .build();

    response
}

// TODO: revisit builder pattern, it feels wrong
pub fn get_object(request: Request) -> Response {
    let mut file = fs::OpenOptions::new().read(true).open(FILE_NAME).unwrap();
    let file_size = file.metadata().unwrap().len() as usize;
    let mut response_builder = ResponseBuilder::new();
    response_builder = response_builder
        // TODO: needs to be configurable
        .header("Content-Type: audio/mpeg")
        .header("Accept-Ranges: bytes");
    // TODO: extract and test range transformation
    let range: Option<(usize, usize)> = request.headers.get("Range").and_then(|range_str| {
        let range = range_str
            .replace("bytes=", "")
            .split("-")
            .map(|r| r.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        let start = range.get(0).copied().unwrap_or(0);
        let end = range.get(1).copied().unwrap_or(file_size);

        // TODO: check why it happens
        if start >= end {
            return None;
        }

        Some((start, end))
    });
    match range {
        Some(range) => {
            let requested_size = range.1 - range.0;
            let mut buffer: Vec<u8> = vec![0; requested_size];
            file.seek(std::io::SeekFrom::Start(range.0 as u64)).unwrap();
            file.read_exact(&mut buffer).unwrap();

            response_builder = response_builder
                .status("206 Partial Content")
                .header(
                    format!("Content-Range: bytes {}-{}/{}", range.0, range.1, file_size).as_str(),
                )
                .file(buffer);
        }
        _ => {
            let mut buffer: Vec<u8> = vec![0; file_size];
            file.read_exact(&mut buffer).unwrap();
            response_builder = response_builder.status("200 OK").file(buffer);
        }
    }

    let response = response_builder.build();

    response
}
