pub struct Server {}

impl Server {
	pub fn new() -> Self {
		Default::default()
	}

	pub fn initialize(&mut self) -> &mut Self {
		self
	}

	pub async fn serve(&mut self) {}
}

impl Default for Server {
	fn default() -> Self {
		Self {}
	}
}
