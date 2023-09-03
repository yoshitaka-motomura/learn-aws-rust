//! # Example: scraping
//! Example of a Lambda function that scrapes a website and returns the title and description.
//! ## Build
//! ```bash
//! cargo lambda build
//! ```
//! ## example 
//! ```bash
//! https://aoi-soragoto.com/scraping?url=https://www.rust-lang.org/
//! ```
//!

use lambda_http::{run, service_fn, Body, Error, Request, RequestExt, Response};
use serde::Serialize;
use url::Url;

mod scraping;
#[derive(Serialize)]
struct JsonResponse {
    url: String,
    meta: scraping::Meta,
}
/// This is the main body for the function.
/// Write your code inside it.
/// There are some code example in the following URLs:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples
async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    // Extract some useful information from the request
    let url = event
        .query_string_parameters_ref()
        .and_then(|params| params.first("url"))
        .unwrap_or("");

    match Url::parse(&url) {
        Err(_) => {
            return Ok(Response::builder()
                .status(400)
                .header("content-type", "text/plain; charset=utf-8")
                .body("Invalid URL format".into())
                .map_err(Box::new)?);
        },
        Ok(_) => {}
    }
   
    let html = match scraping::fetch(url.to_string()).await {
        Ok(html) => html,
        Err(_) => {
            return Ok(Response::builder()
                .status(400)
                .header("content-type", "text/plain; charset=utf-8")
                .body("Failed to fetch the URL".into())
                .map_err(Box::new)?);
        }
    };

    let response = JsonResponse {
        url: url.to_string(),
        meta: html,
    };

    let body = serde_json::to_string(&response)?;
    // Return something that implements IntoResponse.
    // It will be serialized to the right response event automatically by the runtime
    let resp = Response::builder()
        .status(200)
        .header("content-type", "application/json; charset=utf-8")
        .body(body.into())
        .map_err(Box::new)?;
    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    run(service_fn(function_handler)).await
}
