use std::io::Read;
use ureq::Error;

mod objects;
mod services;
mod demo;

pub const BOT_API_URL: &str = "http://localhost:8085/bot"; // "https://api.telegram.org/bot";

pub const HTTP_200_OK: u16 = 200;

enum Method{
	AnswerInlineQuery,
}

struct Request<T> where T: Read {
	method: Method,
	body: T,
}

impl <T>Request<T> where T: Read {
	fn endpoint<'a>(&self) -> &'a str {
		match self.method {
			Method::AnswerInlineQuery => "answerInlineQuery",
		}
	}
}

pub struct Client {
	api_url: String,
	token: String,
}

impl Client {
	fn new(token: &str) -> Client {
		Client{
			api_url: String::from(BOT_API_URL),
			token: String::from(token),
		}
	}

	fn url(&self, endpoint: &str) -> String {
		return format!("{}{}/{}", self.api_url.as_str(), self.token, endpoint)
	}

	fn call<T: Read>(&self, request: Request<T>) -> Result<(), Error>{
		let resp = ureq::post(self.url(request.endpoint()).as_str())
			.set("Content-Type", "application/json")
			.send(request.body);

		match resp {
			Ok(response) => {
				if response.status() == HTTP_200_OK {
					println!("ok")
				}
				Ok(())
			}
			Err(e) => Err(e),
		}
		// Ok(())
	}
}

#[cfg(test)]
mod unit_tests {
	use stringreader::StringReader;
	use crate::{BOT_API_URL, Client, Method, Request};

	#[test]
	fn test_client_new(){
		let client = Client::new("this is a token");
		assert_eq!(BOT_API_URL, client.api_url.as_str());
	}

	#[ignore]
	#[test] // works but disabled
	fn test_client_call(){
		let client = Client::new("this is a test");
		let request = Request{
			body: StringReader::new("this is a test"),
			method: Method::AnswerInlineQuery,
		};
		let result = client.call(request);
		if let Err(e) = result {
			assert!(false, "should not have an issue")
		}
	}
}