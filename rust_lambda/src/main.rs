//! # cargo-lambda example 

use lambda_http::{run, service_fn, Body, Error, Request, RequestExt, Response};
use serde::Serialize; 
extern crate chrono;
use chrono::Local;

#[derive(Serialize)]
struct JsonResponse {
    message: String,
    time: String,
    ip: String,
}

/// This is the main body for the function.
/// Write your code inside it.
/// There are some code example in the following URLs:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples
async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    let who = event
        .query_string_parameters_ref()
        .and_then(|params| params.first("name"))
        .unwrap_or("world");
    
    let ip = match event.request_context() {
        lambda_http::request::RequestContext::ApiGatewayV2(api_gw_context) => {
            api_gw_context.http.source_ip.clone().unwrap_or_else(|| "unknown".to_string())
        },
        _ => "unknown".to_string(),
    };

    let now = Local::now();
    let response = JsonResponse {
        message: format!("Hello, {}", who),
        time: now.format("%Y-%m-%d %H:%M:%S").to_string(),
        ip: ip.to_string(),
    };

    let body = serde_json::to_string(&response)?; // JSONに変換

    let resp = Response::builder()
        .status(200)
        .header("content-type", "application/json")
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
