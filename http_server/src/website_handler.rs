use super::http::{Method, Request, Response, StatusCode};
use super::server::Handler;
use std::fs;
pub struct WebsiteHandler {
    public_path: String,
}

impl WebsiteHandler {
    pub fn new(public_path: String) -> Self {
        WebsiteHandler {
            public_path: public_path,
        }
    }
    fn read_file(&self, file_path: &str) -> Option<String> {
        let path = format!("{}/{}", &self.public_path, file_path);
        match fs::canonicalize(path) {
            Ok(can_path) => {
                if can_path.starts_with(&self.public_path) {
                    fs::read_to_string(can_path).ok()
                } else {
                    println!("Directory Traversal Attack Attempted: {}", file_path);
                    None
                }
            }
            Err(_) => None,
        }
    }
}

impl Handler for WebsiteHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        match request.method() {
            Method::GET => match request.path() {
                "/" => Response::new(StatusCode::Ok, self.read_file("index.html")),
                "/hello" => Response::new(StatusCode::Ok, self.read_file("hello.html")),
                path => match self.read_file(path) {
                    Some(contents) => Response::new(StatusCode::Ok, Some(contents)),
                    None => Response::new(StatusCode::NotFound, None),
                },
            },
            _ => Response::new(StatusCode::NotFound, None),
        }
        //Response::new(StatusCode::Ok, Some("<h1>It works! <h1>".to_string()))
    }
}
