#[tokio::main]
async fn main() {
    let routes_all = Router::new()
        .merge(routes_default())
        // .merge(routes_static())
        .fallback(index);

    let addr = &SocketAddr::new(IpAddr::from(Ipv6Addr::UNSPECIFIED), 3001);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, routes_all).await.unwrap();
}

fn routes_default() -> Router {
    // Router::new()
}
