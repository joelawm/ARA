use std::net::SocketAddr;

pub mod routes;

#[tokio::main]
async fn main() {
	// build our application with a route
	let app = routes::create_routes().await;

	// run it
	let addr: SocketAddr = format!("{}:{}", "127.0.0.1", "3000").parse().expect("Unable to parse socket address");
	let listener = tokio::net::TcpListener::bind(addr).await.expect("Failed to bind listener");
	println!("Server listening on: {}", addr);

	axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>()).await.expect("Failed to run server.");
}