use axum::{Router, response::Html, routing::get};

#[tokio::main]
async fn main() {
  let routes_hello = Router::new().route(
    "/hello",
    get(|| async { Html("<h1>Hello, world BREEEZ!</h1>")})
  );

  let address = "127.0.0.1:8080";

  let listener = tokio::net::TcpListener::bind(address).await.unwrap();
  
  println!("--> LISTENING on {address}...");

  axum::serve(listener, routes_hello).await.unwrap();
}
