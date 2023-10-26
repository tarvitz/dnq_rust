use std::io::Read;
use serde::{Deserialize, Serialize};

mod objects;
mod demo;
mod services;

pub const BOT_API_URL: &str = "https://api.telegram.org/bot";
pub const CONTENT_TYPE_DEFAULT: &str = "application/json";
pub const HTTP_200_OK: u16 = 200;

enum Error {
	NoContents,
}

enum Method{
	AnswerInlineQuery,
}

struct Request<R> where R: Read {
	method: Method,
	body: R,
	expected_status: u16,
}

impl <R>Request<R> where R: Read {
	fn endpoint<'a>(&self) -> &'a str {
		match self.method {
			Method::AnswerInlineQuery => "answerInlineQuery",
		}
	}

	fn new(method: Method, body: R) -> Request<R>{
		Request{
			method, body,
			expected_status: HTTP_200_OK,
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

	fn call<T:Serialize + for<'de> Deserialize<'de>, R:Read>(&self, request: Request<R>) -> Result<T, Error>{
		let resp = ureq::post(self.url(request.endpoint()).as_str())
			.set("Content-Type", CONTENT_TYPE_DEFAULT)
			.send(request.body);

		match resp {
			Ok(response) => {
				if response.status() != request.expected_status {
					return Err(Error::NoContents);
				}

				let result = serde_yaml::from_reader(response.into_reader());
				match result {
					Ok(object) => Ok(object),
					Err(_) => Err(Error::NoContents),
				}
			}
			Err(_) => Err(Error::NoContents),
		}
	}
}

#[cfg(test)]
mod unit_tests {
	use super::*;

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
		let mut client = Client::new("secrettoken");
		client.api_url = String::from("http://localhost:3000/bot");

		let request = Request::new(
			Method::AnswerInlineQuery,
			StringReader::new("this is a test"));

		let result: Result<Update, Error> = client.call(request);
		match result {
			Err(_) => assert!(false, "should not return an issue"),
			Ok(update) => assert_eq!(292124505, update.id),
		}
	}
}