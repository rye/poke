#[tokio::main]
async fn main() {
	pretty_env_logger::init();

	let mut server = slowpoke::Server::new();
	server.initialize();
	server.serve().await
}
