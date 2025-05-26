use std::env;
use std::process::exit;

use hyper::{Client, http::StatusCode};

#[tokio::main]
async fn main() {
    let port = match env::var("PORT") {
        Ok(p) => p,
        Err(_) => String::from("8080"),
    };

    let path = env::var("HEALTHCHECK_PATH").unwrap_or_default();

    let client = Client::new();

    let url = format!("http://localhost:{port}{path}").parse().unwrap();
    let res = client.get(url).await;

    res.map(|res| {
        let status_code = res.status();
        if status_code < StatusCode::from_u16(200).unwrap()
            || status_code > StatusCode::from_u16(399).unwrap()
        {
            exit(1)
        }
        exit(0)
    })
    .map_err(|_| exit(1))
    .ok();
}
