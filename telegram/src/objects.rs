use std::collections::HashMap;
use std::sync::{Mutex, Once};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

static mut QUOTES: Option<Mutex<HashMap<String, Vec<Quote>>>> = None;
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
	pub caption: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Quote {
	pub id: String,
	pub caption: String,
	pub matches: Vec<String>,
}

// init runs initialization call
pub fn quotes<'a>() -> &'static Mutex<HashMap<String, Vec<Quote>>>{
	INIT.call_once(||{
		unsafe {
			let quotes: HashMap<String, Vec<Quote>> = HashMap::new();
			QUOTES = Some(Mutex::new(quotes));
		}
	});
	unsafe { QUOTES.as_ref().unwrap() } // remote Option
}

pub fn new_answer_inline(update: &Update) -> AnswerInline {
	let mut id: String = String::new();
	if let Some(q) = &update.inline_query {
		id = q.id.clone();
	}

	return AnswerInline{
		id,
		results: new_inline_query_result_cached_voice(update),
	}
}

fn new_inline_query_result_cached_voice(update: &Update) -> Vec<AnswerInlineResult> {
	let mut query: String = ".X".to_string();
	if let Some(q) = &update.inline_query {
		query = q.query.to_lowercase();
	}
	let mut results:Vec<AnswerInlineResult> = vec![];
	let quotes = quotes().lock().unwrap();
	let q: Option<&Vec<Quote>>; // = None;

	if !quotes.contains_key(&query) {
		q = quotes.get(""); // default quote
	} else {
		q = quotes.get(&query);
	}

	for quote in q.unwrap().iter() {
		results.push(AnswerInlineResult{
			id: Uuid::new_v4().to_string(),
			r#type: "voice".to_string(),
			caption: quote.caption.clone(),
			title: quote.caption.clone(),
			voice_file_id: Some(quote.id.clone()),
		})
	}

	results

}

// TODO: think about decreasing clone ops
pub fn set_quotes(sources: &Vec<Quote>){
	let mut quotes = quotes().lock().unwrap();

	for quote in sources {
		for match_key in quote.matches.iter(){
			if !quotes.contains_key(match_key.as_str()) {
				quotes.insert(match_key.clone(), vec![]);
			}
			quotes.get_mut(match_key.as_str()).unwrap().push(quote.clone());
		}
	}

	return
}

#[cfg(test)]
mod unit_tests {
	use super::*;

	static TEST_INIT: Once = Once::new();

	// using quotes but with initial item initializing
	fn test_quotes() -> &'static Mutex<HashMap<String, Vec<Quote>>>{
		let mut quotes = quotes();

		TEST_INIT.call_once(move ||{
			// add default
			quotes.lock().unwrap().insert("".to_string(), vec![Quote {
				id: "AwACAgIAAxkDAAMRX3J5RTS6ijieWbCFrX68h8-o4ZoAAg8HAAKguZhLPoXn4iWWE2QbBA".to_string(),
				caption: "Come get some!".to_string(),
				matches: vec!["come", "get", "some", "come get some", ""].
					iter().map(|x| x.to_string()).collect(),
			}]);
		});

		quotes
	}

	// TODO: think about how to keep global quotes.
	#[test]
	fn test_new_inline_query_result_cached_voice(){
		// release lock.
		{
			let mut quotes = test_quotes().lock().unwrap();

			quotes.insert("hehe".to_string(), vec![
				Quote {
					id: "test".to_string(),
					caption: "value".to_string(),
					matches: vec!["hehe".to_string()]
				}
			]);
		}

		let update = Update{
			id: 1337,
			inline_query: Some(InlineQuery{
				id: "133733".to_string(),
				query: "hehe".to_string(),
				from: None,
				offset: String::new(),
			}),
			message: None,
		};

		let answer = new_inline_query_result_cached_voice(&update);
		assert_eq!(1, answer.len(), "answer should be have one item");
	}

	#[test]
	fn test_set_qs(){
		let new_quotes = vec![
			Quote { id: "1".to_string(), caption: "".to_string(), matches: vec!["test".to_string()] },
			Quote { id: "2".to_string(), caption: "".to_string(), matches: vec!["me".to_string()] },
		];

		// TODO: we require to calculate the current size of QUOTES object before running
		//       the test, since we can't guarantee tests running order.
		//	     Consider to leave it unchanged or simplify somehow.
		let get_expected= || -> usize {
			return test_quotes().lock().unwrap().len();
		};
		let expected = get_expected();

		set_quotes(&new_quotes);
		let q = test_quotes().lock().unwrap();
		assert_eq!(expected + 2, q.len());
	}
}