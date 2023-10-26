use std::io::Read;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Container<'a> {
	id: &'a str,
	contents: Vec<&'a str>
}

#[cfg(test)]
mod unit_tests {
	use std::collections::HashMap;
	use super::*;

	#[test]
	fn test_internal_object(){
		let container = Container{id: "test", contents: vec!["this", "is", "test"]};
		println!("container: {:?}", container);
	}

	#[test]
	fn test_copy(){

		fn copy<'a>(mut to: HashMap<&'a str, Vec<Container<'a>>>, sources: Vec<Container<'a>>) -> HashMap<&'a str, Vec<Container<'a>>>{
			for container in sources {
				if !to.contains_key(container.id) {
					to.insert(container.id, Vec::new());
				}
				to.get_mut(container.id).unwrap().push(container);
			}

			to
		}

		let map : HashMap<&str, Vec<Container>> = HashMap::new();
		let sources = vec![
			Container{id: "test", contents:vec!["this", "is", "a", "test"]},
			Container{id: "me", contents:vec!["me", "mine"]},
		];
		let map = copy(map, sources);
		assert_eq!(2, map.len());
	}

	#[test]
	fn from_string(){
		let raw = String::from(r#"
id: "this is the test"
contents: ["test", "me"]
"#);
		let obj:Container = serde_yaml::from_str(raw.as_str()).unwrap();
		println!("This is container: {:?}", obj);
	}
}

#[derive(Debug)]
struct Error {
	message: String,
}

struct Request<'a, R> where R:Read {
	method: &'a str,
	body: R,
}

struct Poster<'a> {
	api_url: &'a str,
	token: &'a str,
}

impl <'a>Poster<'a> {

	fn endpoint(&self, method: &str) -> String {
		return format!("{}{}/{}", self.api_url, self.token, method)
	}

	fn call<T:Serialize + for<'de> Deserialize<'de>, R:Read>(&self, request: Request<R>) -> Result<T, Error> {
		let result = ureq::post(self.endpoint(request.method).as_str())
			.send(request.body);

		return match result {
			Ok(response) => {
				let res = serde_yaml::from_reader(response.into_reader());
				match res {
					Ok(obj) => Ok(obj),
					Err(_) => Err(Error { message: String::from("issue") }),
				}
			},
			Err(_) => Err(Error { message: String::from("kek") }),
		};
	}
}

#[cfg(test)]
mod unit_tests_two {
	use stringreader::StringReader;
	use crate::objects::Update;
	use super::*;

	#[test]
	fn test_call_one() {
		let client = Poster{api_url: "http://localhost:3000/bot", token: "secrettoken"};
		let body = StringReader::new("this is a test");
		let result: Result<Update, Error> = client.call(Request{body, method: "answerInlineQuery"});
		match result {
			Ok(update) => {
				assert_eq!(292124505, update.id)
			},
			Err(e) => assert!(false, "{}", format!("got issue: {}", e.message)),
		}
	}
}