extern crate hyper;

use hyper::http::StatusCode;
use hyper::rt::{self, Future};
use hyper::Client;
use std::env;
use std::process::exit;

fn main() {
    rt::run(rt::lazy(|| {
        let port = match env::var("PORT") {
            Ok(p) => p,
            Err(_) => String::from("8080"),
        };
        let path = match env::var("HEALTHCHECK_PATH") {
            Ok(p) => p,
            Err(_) => String::from(""),
        };

        let client = Client::new();
        let url = format!("http://localhost:{}{}", port, path)
            .parse()
            .unwrap();
        client
            .get(url)
            .map(|res| {
                let status_code = res.status();
                if status_code < StatusCode::from_u16(200).unwrap()
                    || status_code > StatusCode::from_u16(399).unwrap()
                {
                    exit(1)
                }
                exit(0)
            })
            .map_err(|_| exit(1))
    }));
}
