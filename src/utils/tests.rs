use std::env;

pub struct WithEnv<'a>{
	key: &'a str,
}

impl Drop for WithEnv<'_> {
	fn drop(&mut self) {
		env::remove_var(self.key)
	}
}

impl <'a>WithEnv<'a> {
	pub fn set(key: &'a str, value: &'a str) -> WithEnv<'a> {
		env::set_var(&key, value);

		WithEnv{
			key,
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
}