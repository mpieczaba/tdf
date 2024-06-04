mod utils;

#[tokio::main]
async fn main() {
	let file = std::env::args()
		.nth(1)
		.expect("Please enter a file to profile");

	utils::render_doc(file).await;
}
