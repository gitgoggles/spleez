use axum::{
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use std::net::{IpAddr, Ipv6Addr, SocketAddr};

#[tokio::main]
async fn main() {
    let routes_all = Router::new().merge(routes_default()).fallback(oops);

    let addr = &SocketAddr::new(IpAddr::from(Ipv6Addr::UNSPECIFIED), 3001);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, routes_all).await.unwrap();
}

fn routes_default() -> Router {
    Router::new()
        .route("/", get(index))
        .route("/favicon.ico", get(favicon))
        .route("/hello", get(hello))
        .route("/oops", get(oops))
        .route("/tailwind-output.css", get(tailwind))
}

// handlers

async fn index() -> impl IntoResponse {
    let content = include_str!("../web_assets/index.html");
    Html(content)
}
async fn tailwind() -> impl IntoResponse {
    let content = include_str!("../web_assets/tailwind-output.css");
    ([(axum::http::header::CONTENT_TYPE, "text/css")], content)
}

async fn favicon() -> impl IntoResponse {
    include_bytes!("../web_assets/favicon.ico")
}

async fn hello() -> String {
    let greeting = "hello".to_string();
    format!("{:?}", greeting)
}

async fn oops() -> impl IntoResponse {
    Html("Where were you trying to get to?")
}
