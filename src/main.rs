use axum::{Router, response::{Html, IntoResponse}, routing::get, extract::Query};
use serde::Deserialize;

#[tokio::main]
async fn main() {
  let routes_hello = Router::new().route(
    "/hello",
    get(handler_hello)
  );

  let address = "127.0.0.1:8080";

  let listener = tokio::net::TcpListener::bind(address).await.unwrap();
  
  println!("--> LISTENING on {address}...");

  axum::serve(listener, routes_hello).await.unwrap();
}

#[derive(Debug, Deserialize)]  
struct HelloParams {
  name: Option<String>
}

async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
  print!("--> {:<12} - handler_hello - {params:?}", "HANDLER");

  let name = params.name.as_deref().unwrap_or("World!");
  Html(format!("Hello <strong>{name}</strong>!"))
}