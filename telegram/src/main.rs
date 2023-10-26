use ureq::{get, head};

fn main() -> Result<(), ureq::Error>{
	println!("kek");
	match head("https://google.com").
		set("User-Agent", "pytho-rust").call() {
		Ok(response) => {
			println!("response: {:?}", response.headers_names());
		},
		Err(e) => {
			println!("got error: {}", e);
		}
	}

	Ok(())
}