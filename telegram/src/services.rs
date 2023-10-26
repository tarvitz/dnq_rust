use stringreader::StringReader;
use crate::{Client, Error, Method, Request};
use crate::objects::{new_answer_inline, Update};

struct Inline<'a> {
	client: &'a Client
}

impl Inline<'_> {
	// TODO: replace result with valid telegram response (it's not json!)
	fn answer_inline_query(&self, update: &Update) -> Result<Update, Error>{
		let answer = new_answer_inline(update);
		let result = serde_json::to_string(&answer);

		if let Ok(contents) = result {
			let request = Request::new(
				Method::AnswerInlineQuery,
				StringReader::new(contents.as_str()),
			);

			return self.client.call(request);
		}

		return Err(Error::NoContents);
	}
}

#[cfg(test)]
mod unit_tests {
	use crate::Client;
	use super::*;

	#[test]
	fn test_inline_answer_inline_query() {
		let mut client = Client::new("secrettoken");
		client.api_url = String::from("http://localhost:3000/bot");
		let inline = Inline{client: &client};
		let result: Result<Update, Error> = inline.answer_inline_query(&Update::default());
		match result {
			Ok(up) => assert_eq!(292124505, up.id),
			Err(_) => assert!(false, "got error"),
		}
	}
}