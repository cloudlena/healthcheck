use http_body_util::Empty;
use hyper::Request;
use hyper::StatusCode;
use hyper::body::Bytes;
use hyper_util::client::legacy::Client;
use hyper_util::rt::TokioExecutor;

use std::env;
use std::process::exit;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let port = match env::var("PORT") {
        Ok(p) => p,
        Err(_) => String::from("8080"),
    };

    let path = env::var("HEALTHCHECK_PATH").unwrap_or_default();

    let url = format!("http://localhost:{port}{path}").parse::<hyper::Uri>()?;

    let authority = url.authority().unwrap().clone();

    // Create an HTTP request with an empty body and a HOST header
    let req = Request::builder()
        .uri(url)
        .header(hyper::header::HOST, authority.as_str())
        .body(Empty::<Bytes>::new())?;

    let client = Client::builder(TokioExecutor::new()).build_http();

    // Await the response...
    let response = client.request(req).await?;

    let status_code = response.status();
    if status_code < StatusCode::from_u16(200).unwrap()
        || status_code > StatusCode::from_u16(399).unwrap()
    {
        exit(1)
    }
    exit(0)
}
