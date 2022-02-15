#[tokio::main]
async fn main() {
	tracing_subscriber::fmt::init();

	let mut server = slowpoke::Server::new();
	server.initialize();
	server.serve().await
}
