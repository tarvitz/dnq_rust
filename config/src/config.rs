use std::{fs, vec};
use rand::Rng;
use serde::{Deserialize, Serialize};


const DEFAULT_QUOTE_ID: &str = "AwACAgIAAxkDAAMWX3okXL1AZ-aOQTpL2t-7tExt2YIAArMIAAJgG9BLmWJEVGtI5hwbBA";

pub struct Error<'a> {
	pub message: &'a str,
}

// Quote -> telegram
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Quote {
	pub id: String,
	pub caption: String,
	pub matches: Vec<String>,
}

impl Clone for Quote {
	fn clone(&self) -> Self {
		return Quote{
			id: self.id.clone(),
			caption: self.caption.clone(),
			matches: self.matches.clone(),
		}
	}
}

// note fields should have exact names like they represented in structs
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
	// see also: https://serde.rs/field-attrs.html
	#[serde(alias = "admin-token")]
	pub admin_token: String,
	pub quotes: Vec<Quote>
}

impl Config {
	pub fn from_file(filename: &str) -> Result<Config, Error> {
		if let Ok(contents) = fs::read_to_string(filename) {
			if let Ok(cfg) = serde_yaml::from_str(contents.as_str()) {
				return Ok(cfg);
			}
		}
		return Err(Error { message: "could get config" })
	}

	// TODO: probably there's a way to find a way without cloning?
	pub fn random_quote(&self) -> Quote {
		let len= self.quotes.len();
		if len == 0 {
			return default_quote();
		}
		if len == 1 {
			return self.quotes.get(0).unwrap().clone();
		}

		let idx = rand::thread_rng().gen_range(0..len-1);
		match self.quotes.get(idx){
			Some(v) => {
				return v.clone();
			},
			_ => default_quote(),
		}
	}
}

// helpers
fn default_quote() -> Quote {
	Quote{
		id: String::from(DEFAULT_QUOTE_ID),
		caption: String::from("What the ..?"),
		matches: vec![],
	}
}

#[cfg(test)]
mod test_config {
	use std::panic::panic_any;
	use super::*;

	const CONFIG_LOCATION: &str = "tests/config.yaml";

	// helpers
	fn _make_blank_config() -> Config {
		Config {
			quotes: vec![],
			admin_token: String::from("thisIsSecret"),
		}
	}

	#[test]
	fn from_file() {
		if let Ok(config) = Config::from_file(CONFIG_LOCATION) {
			assert_eq!(String::from("thisIsSecret"), config.admin_token);
			assert_eq!(4, config.quotes.len());
			assert_eq!(String::from("AwACAgIAAxkDAAMRX3"), config.quotes[0].id);
			return
		};
		panic_any("could not deserialize the config");
	}

	#[test]
	fn from_file_does_not_exist() {
		if let Err(err) = Config::from_file("does not exist") {
			assert_eq!(String::from("could get config"), err.message);
			return
		};
		panic_any("didn't return an error");
	}

	#[test]
	fn test_random_quote_blank() {
		let config = _make_blank_config();
		// let quote = *config.random_quote().deref();
		let quote = config.random_quote();
		assert_eq!(default_quote(), quote);
	}

	#[test]
	fn test_random_quote_non_blank() {
		let mut config = _make_blank_config();
		config.quotes.push(Quote{
			id: String::from("1"),
			caption: String::from("2"),
			matches: vec![],
		});

		let quote = config.random_quote();
		assert_ne!(default_quote(), quote);

		config.quotes.push(Quote{
			id: String::from("2"),
			caption: String::from("1"),
			matches: vec![],
		});

		let quote = config.random_quote();
		assert_ne!(default_quote(), quote);
	}
}