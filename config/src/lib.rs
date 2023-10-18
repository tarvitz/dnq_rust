use std::fs;
use serde::{Deserialize, Serialize};

pub struct Error<'a> {
	pub message: &'a str,
}

// Quote -> telegram
#[derive(Debug, Serialize, Deserialize)]
pub struct Quote {

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
		if let Ok(contents) = fs::read_to_string(filename){
			if let Ok(cfg) = serde_yaml::from_str(contents.as_str()) {
				return Ok(cfg);
			}
		}
		return Err(Error{message: "could get config"})
	}
}

#[cfg(test)]
mod test_config {
	use std::panic::panic_any;
	use super::*;

	const CONFIG_LOCATION: &str = "tests/config.yaml";

	#[test]
	fn from_file() {
		if let Ok(config) = Config::from_file(CONFIG_LOCATION){
			assert_eq!(String::from("thisIsSecret"), config.admin_token);
			assert_eq!(4, config.quotes.len());
			return
		};
		panic_any("could not deserialize the config");
	}

	#[test]
	fn from_file_does_not_exist() {
		if let Err(err) = Config::from_file("does not exist"){
			assert_eq!(String::from("could get config"), err.message);
			return
		};
		panic_any("didn't return an error");
	}
}
