#[tokio::main]
async fn main() {
	let mut server = poke::Server::new();
	server.initialize();
	server.serve().await
}
