use std::io::Read;
use serde::Deserialize;
use crate::objects::Update;

mod objects;
mod services;
mod demo;

pub const BOT_API_URL: &str = "http://localhost:3000/bot"; // "https://api.telegram.org/bot";

pub const HTTP_200_OK: u16 = 200;

enum Error {
	NoContents,
}

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
				if response.status() != HTTP_200_OK {
					return Err(Error::NoContents);
				}
				println!("ok");
				let result: Update = serde_yaml::from_reader(response.into_reader()).unwrap();
				println!("result: {:?}", result);
				Ok(())
				// let mut update = Update::default();
				// let result = serde_yaml::from_reader(response.into_reader());
				//
				// match result{
				// 	Ok(u) => return Ok(Box::new(u)),
				// 	Err(_) => return Error::NoContents,
				// }
				// // Box::new(update)
				// // Ok(Box::new(update))
			}
			Err(e) => Err(Error::NoContents),
		}
		// Ok(())
	}
}

#[cfg(test)]
mod unit_tests {
	use stringreader::StringReader;
	use crate::{BOT_API_URL, Client, Method, Request};
	use crate::objects::Update;

	#[test]
	fn test_client_new(){
		let client = Client::new("this is a token");
		assert_eq!(BOT_API_URL, client.api_url.as_str());
	}

	#[ignore] // at the present moment this test runs on top of mockoon running side-server.
	#[test] // works but disabled
	fn test_client_call(){
		let client = Client::new("secrettoken");
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