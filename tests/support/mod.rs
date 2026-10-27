// The blocking-only build uses a subset of these helpers.
#![allow(dead_code)]

use base64::Engine;
use base64::engine::general_purpose::STANDARD;
use chrono::{DateTime, NaiveDate, Utc};
use identisure_client::PollOptions;
use serde_json::Value;
use std::collections::VecDeque;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub const CLIENT_ID: &str = "client-id";
pub const CLIENT_SECRET: &str = "client-secret";

fn fixture_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("fixtures")
        .join(name)
}

pub fn fixture(name: &str) -> String {
    std::fs::read_to_string(fixture_path(name)).unwrap_or_else(|e| panic!("{name}: {e}"))
}

pub fn fixture_json(name: &str) -> Value {
    serde_json::from_str(&fixture(name)).unwrap()
}

pub fn str_at<'a>(value: &'a Value, key: &str) -> &'a str {
    value[key]
        .as_str()
        .unwrap_or_else(|| panic!("{key} is not a string"))
}

pub fn date(value: &Value, key: &str) -> NaiveDate {
    str_at(value, key).parse().unwrap()
}

pub fn instant(value: &Value, key: &str) -> DateTime<Utc> {
    str_at(value, key).parse().unwrap()
}

pub fn poll() -> PollOptions {
    PollOptions {
        interval: Duration::from_millis(1),
        timeout: Duration::from_secs(5),
    }
}

#[derive(Debug)]
pub struct RecordedRequest {
    pub method: String,
    pub path: String,
    pub authorization: Option<String>,
    pub content_type: Option<String>,
    pub body: Vec<u8>,
}

/// Local HTTP server that answers each request with the next queued fixture.
pub struct MockServer {
    url: String,
    requests: Arc<Mutex<Vec<RecordedRequest>>>,
}

impl MockServer {
    /// `responses` are `(status, fixture name)` pairs served in order.
    pub fn start(responses: &[(u16, &str)]) -> Self {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let url = format!("http://{}", listener.local_addr().unwrap());
        let queue = Arc::new(Mutex::new(
            responses
                .iter()
                .map(|&(status, name)| (status, fixture(name)))
                .collect::<VecDeque<_>>(),
        ));
        let requests = Arc::new(Mutex::new(Vec::new()));
        let recorded = Arc::clone(&requests);

        thread::spawn(move || {
            for stream in listener.incoming() {
                let Ok(stream) = stream else {
                    return;
                };
                let queue = Arc::clone(&queue);
                let recorded = Arc::clone(&recorded);

                thread::spawn(move || serve(stream, &queue, &recorded));
            }
        });

        Self { url, requests }
    }

    pub fn url(&self) -> &str {
        &self.url
    }

    pub fn requests(&self) -> std::sync::MutexGuard<'_, Vec<RecordedRequest>> {
        self.requests.lock().unwrap()
    }

    /// Asserts the method and path of every request and that each carried the client's Basic auth.
    pub fn assert_requests(&self, expected: &[(&str, &str)]) {
        let requests = self.requests();
        let actual = requests
            .iter()
            .map(|r| (r.method.as_str(), r.path.as_str()))
            .collect::<Vec<_>>();
        let auth = format!(
            "Basic {}",
            STANDARD.encode(format!("{CLIENT_ID}:{CLIENT_SECRET}"))
        );

        assert_eq!(actual, expected);

        for request in requests.iter() {
            assert_eq!(request.authorization.as_deref(), Some(auth.as_str()));
        }
    }
}

fn serve(
    stream: TcpStream,
    queue: &Mutex<VecDeque<(u16, String)>>,
    recorded: &Mutex<Vec<RecordedRequest>>,
) {
    let mut reader = BufReader::new(stream.try_clone().unwrap());
    let mut writer = stream;

    loop {
        let mut request_line = String::new();

        if reader.read_line(&mut request_line).unwrap_or(0) == 0 {
            return;
        }

        let mut parts = request_line.split_whitespace();
        let method = parts.next().unwrap_or_default().to_owned();
        let path = parts.next().unwrap_or_default().to_owned();
        let mut authorization = None;
        let mut content_type = None;
        let mut content_length = 0;

        loop {
            let mut line = String::new();

            reader.read_line(&mut line).unwrap();

            let line = line.trim_end();

            if line.is_empty() {
                break;
            }

            let (name, value) = line.split_once(':').unwrap();
            let value = value.trim().to_owned();

            match name.to_ascii_lowercase().as_str() {
                "authorization" => authorization = Some(value),
                "content-type" => content_type = Some(value),
                "content-length" => content_length = value.parse().unwrap(),
                _ => {},
            }
        }

        let mut body = vec![0; content_length];

        reader.read_exact(&mut body).unwrap();
        recorded.lock().unwrap().push(RecordedRequest {
            method,
            path,
            authorization,
            content_type,
            body,
        });

        let (status, body) = queue
            .lock()
            .unwrap()
            .pop_front()
            .unwrap_or((500, r#"{"message":"No response queued"}"#.to_owned()));
        let response = format!(
            "HTTP/1.1 {status} Mock\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{body}",
            body.len()
        );

        if writer.write_all(response.as_bytes()).is_err() {
            return;
        }
    }
}
