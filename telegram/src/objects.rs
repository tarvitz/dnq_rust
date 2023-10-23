use std::collections::HashMap;
use std::sync::{Mutex, Once};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

static mut QUOTES: Option<Mutex<HashMap<&str, Vec<Quote>>>> = None;
static INIT: Once = Once::new();


#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Response {
	pub status: bool,
	pub message: Option<Message>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Message {
	#[serde(alias = "message_id")]
	pub id: i64,
	pub from: Option<From>,
	pub chat: Option<From>,
	pub date: i64,
	pub text: String,
	pub voice: Option<Voice>
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct From {
	#[serde(alias = "message_id")]
	pub id: i64,
	pub r#type: String,
	pub is_bot: bool,
	pub first_name: String,
	pub last_name: String,
	pub username: String,
	pub language_code: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Voice {
	pub duration: i32,
	pub mime_type: String,
	pub file_id: String,
	pub file_unique_id: String,
	pub file_size: usize,
}

enum UpdateType {
	Unknown,
	Inline,
	Message,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Update {
	#[serde(alias = "message_id")]
	pub id: i64,
	pub message: Option<Message>,
	pub inline_query: Option<InlineQuery>,
}

impl Update {
	pub fn r#type(&self) -> UpdateType {
		if let Some(_) = &self.message {
			return UpdateType::Message;
		}
		if let Some(_) = &self.inline_query {
			return UpdateType::Inline;
		}

		UpdateType::Unknown
	}
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct InlineQuery {
	pub id: String,
	pub from: Option<From>,
	pub query: String,
	pub offset: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AnswerInline {
	#[serde(alias = "inline_query_id")]
	pub id: String,
	pub results: Vec<AnswerInlineResult>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AnswerInlineResult {
	#[serde(alias = "type")]
	pub r#type: String,
	pub id: String,
	pub voice_file_id: Option<String>,
	pub title: String,
	pub caption: String
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Quote {
	pub id: String,
	pub caption: String,
	pub matches: Vec<String>,
}

// init runs initialization call
fn quotes<'a>() -> &'static Mutex<HashMap<&'a str, Vec<Quote>>>{
	INIT.call_once(||{
		unsafe {
			QUOTES = Some(Mutex::new(HashMap::new()));
		}
	});
	unsafe { QUOTES.as_ref().unwrap() }
}

pub fn new_answer_inline(update: &Update) -> AnswerInline {
	let mut id:String = String::new();
	if let Some(q) = &update.inline_query {
		id = q.id.clone();
	}
	return AnswerInline{
		id,
		results: new_inline_query_result_cached_voice(update),
	}
}

fn new_inline_query_result_cached_voice(update: &Update) -> Vec<AnswerInlineResult> {
	let mut query: String = String::from(".X");
	if let Some(q) = &update.inline_query {
		query = q.query.to_lowercase();
	}
	let mut results:Vec<AnswerInlineResult> = vec![];
	let quotes = quotes().lock().unwrap();
	// let quotes: HashMap<&str, Vec<Quote>> = HashMap::new();
	let mut q: Option<&Vec<Quote>> = None;

	if !quotes.contains_key(&query.as_str()) {
		q = quotes.get(""); // default quote
	} else {
		q = quotes.get(&query.as_str());
	}
	for quote in q.unwrap().iter() {
		results.push(AnswerInlineResult{
			id: Uuid::new_v4().to_string(),
			r#type: String::from("voice"),
			caption: quote.caption.clone(),
			title: quote.caption.clone(),
			voice_file_id: Some(quote.id.clone()),
		})
	}

	results

}

// TODO: think about decreasing clone ops
pub fn set_quotes(quotes: Vec<Quote>) {
	for quote in quotes {
		for r#match in quote.matches.iter() {
			// let vec = QUOTES.get_mut(r#match.as_str()).unwrap();
			// vec.push(&quote);
		}
	}
}

#[cfg(test)]
mod unit_tests {
	use crate::objects::{InlineQuery, new_inline_query_result_cached_voice, Quote, quotes, Update};

	#[test]
	fn setup(){
		assert!(true);
	}

	// TODO: think about how to keep global quotes.
	#[test]
	fn test_new_inline_query_result_cached_voice(){
		{
			let mut quotes = quotes().lock().unwrap();

			quotes.insert("hehe", vec![Quote {
				id: String::from("test"),
				caption: String::from("value"),
				matches: vec![String::from("hehe")]
			}]);
		}

		let update = Update{
			id: 1337,
			inline_query: Some(InlineQuery{
				id: String::from("133733"),
				query: String::from("hehe"),
				from: None,
				offset: String::new(),
			}),
			message: None,
		};

		let answer = new_inline_query_result_cached_voice(&update);
		assert_eq!(1, answer.len(), "answer should be have one item");
	}
}