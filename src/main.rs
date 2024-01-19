use axum::{Router, response::{Html, IntoResponse}, routing::get};

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

async fn handler_hello() -> impl IntoResponse {
  print!("--> {:<12} - handler_hello", "HANDLER");

  Html("Hello <strong>World</strong>!")
}