use std::env;

pub struct WithEnv<'a>{
	key: &'a str,
	stored: Option<String>,
}

impl Drop for WithEnv<'_> {
	fn drop(&mut self) {
		match &self.stored {
			Some(value) => {
				env::set_var(self.key, value);
			}
			None => env::remove_var(self.key)
		}
	}
}

impl <'a>WithEnv<'a> {
	pub fn set(key: &'a str, value: &'a str) -> WithEnv<'a> {
		match env::var(key){
			Ok(value) => {
				env::set_var(&key, &value);
				WithEnv{key, stored: Some(value)}
			},
			Err(_) => {
				env::set_var(&key, value);
				WithEnv{key, stored: None}
			}
		}
	}

	pub fn run<F: FnOnce()>(self, run: F) {
		run()
	}
}

#[cfg(test)]
mod unit_tests {
	use std::env;
	use crate::utils::tests::WithEnv;

	#[test]
	fn test_set(){
		WithEnv::set("TEST_KEY", "value").run(||{
			println!("test");
			let result = env::var("TEST_KEY").unwrap();
			assert_eq!("value", result);
		});

		let result = env::var("TEST_KEY").
			unwrap_or(String::from(""));
		assert_eq!("", result);
	}

	#[test]
	fn test_override(){
		env::set_var("TEST_VAR", "test value");
		WithEnv::set("TEST_VAR", "1337").run(||{});
		assert_eq!(String::from("test value"),
							 env::var("TEST_VAR").unwrap_or(String::from("")));
	}
}