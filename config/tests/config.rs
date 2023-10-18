use config::Config;

const CONFIG_LOCATION: &str = "tests/config.yaml";

#[cfg(test)]
mod i_test_config{
	use std::panic::panic_any;
	use super::*;

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
