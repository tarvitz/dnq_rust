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
	pub id: i64,
	#[serde(alias = "type")]
	pub r#type: Option<String>,
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

pub enum UpdateType {
	Unknown,
	Inline,
	Message,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Update {
	#[serde(alias = "update_id")]
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

	pub fn default() -> Update {
		Update{
			id: 0,
			message: None,
			inline_query: None,
		}
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
pub struct AnswerInline<'a> {
	#[serde(alias = "inline_query_id")]
	pub id: &'a str,
	pub results: Vec<AnswerInlineResult<'a>>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AnswerInlineResult<'a> {
	#[serde(alias = "type")]
	pub r#type: &'a str,
	pub id: String,
	pub voice_file_id: Option<&'a str>,
	pub title: &'a str,
	pub caption: &'a str
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Quote<'a> {
	pub id: &'a str,
	pub caption: &'a str,
	pub matches: Vec<&'a str>,
}

// init runs initialization call
fn quotes<'a>() -> &'static Mutex<HashMap<&'a str, Vec<Quote<'static>>>>{
	INIT.call_once(||{
		unsafe {
			let mut quotes: HashMap<&'a str, Vec<Quote>> = HashMap::new();
			// add default
			quotes.insert("", vec![Quote{
				id: "AwACAgIAAxkDAAMRX3J5RTS6ijieWbCFrX68h8-o4ZoAAg8HAAKguZhLPoXn4iWWE2QbBA",
				caption: "Come get some!",
				matches: vec!["come", "get", "some", "come get some", ""],
			}]);
			QUOTES = Some(Mutex::new(quotes));
		}
	});
	unsafe { QUOTES.as_ref().unwrap() } // remote Option
}

pub fn new_answer_inline(update: &Update) -> AnswerInline {
	let mut id:&str = "";
	if let Some(q) = &update.inline_query {
		id = q.id.as_str();
	}

	return AnswerInline{
		id,
		results: new_inline_query_result_cached_voice(update),
	}
}

fn new_inline_query_result_cached_voice<'a>(update: &Update) -> Vec<AnswerInlineResult<'a>> {
	let mut query: String = String::from(".X");
	if let Some(q) = &update.inline_query {
		query = q.query.to_lowercase();
	}
	let mut results:Vec<AnswerInlineResult> = vec![];
	let quotes = quotes().lock().unwrap();
	let q: Option<&Vec<Quote>>; // = None;

	if !quotes.contains_key(&query.as_str()) {
		q = quotes.get(""); // default quote
	} else {
		q = quotes.get(&query.as_str());
	}

	for quote in q.unwrap().iter() {
		results.push(AnswerInlineResult{
			id: Uuid::new_v4().to_string(),
			r#type: "voice",
			caption: quote.caption,
			title: quote.caption,
			voice_file_id: Some(quote.id),
		})
	}

	results

}

// TODO: think about decreasing clone ops
// note Quote<'static> here is used because we manipulate with quotes, which uses 'static!
pub fn set_quotes(sources: &Vec<Quote<'static>>) {
	let mut quotes = quotes().lock().unwrap();

	for quote in sources {
		for match_key in quote.matches.iter() {
			if !quotes.contains_key(match_key) {
				quotes.insert(match_key, vec![]); // init vector
			}
			quotes.get_mut(match_key).unwrap().push(quote.clone()); // append vector values

		}
	}

	return
}

#[cfg(test)]
mod unit_tests {
	use crate::objects::{InlineQuery, new_inline_query_result_cached_voice, Quote, quotes, set_quotes, Update};

	#[test]
	fn setup(){
		assert!(true);
	}

	// TODO: think about how to keep global quotes.
	#[test]
	fn test_new_inline_query_result_cached_voice(){
		// release lock.
		{
			let mut quotes = quotes().lock().unwrap();

			quotes.insert("hehe", vec![
				Quote{
					id: "test",
					caption: "value",
					matches: vec!["hehe"]
				}
			]);
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

	#[test]
	fn test_set_quotes(){
		let new_quotes = vec![
			Quote{ id: "1", caption: "", matches: vec!["test"] },
			Quote{ id: "2", caption: "", matches: vec!["me"] },
		];

		// TODO: we require to calculate the current size of QUOTES object before running
		//       the test, since we can't guarantee tests running order.
		//	     Consider to leave it unchanged or simplify somehow.
		let get_expected= || -> usize {
			return quotes().lock().unwrap().len();
		};
		let expected = get_expected();

		set_quotes(&new_quotes);
		let q = quotes().lock().unwrap();
		assert_eq!(expected + 2, q.len());
	}
}