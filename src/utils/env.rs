use std::env as stdenv;
use std::str::FromStr;

/// Env provides a way to fetch a value from
/// environment variables and set a default value on any issue
/// # Example
/// ```
/// use dnq::utils::env::Env;
/// let result = Env::with("TEST_VALUE", 1337u32).get();
/// assert_eq!(1337u32, result);
/// ```
pub struct Env<'a, T>
	where T: FromStr {

	key: &'a str,
	value: T,
}

impl <'a, T> Env<'a, T>
	where T:FromStr {

	/// with sets `key` as an environment key to store
	/// `value` is used to set default value.
	pub fn with(key: &str, value: T) -> Env<T>{
		Env {key, value}
	}

	/// get processes environment variable stored in `self.key`
	/// if environment variable is found and can be parsed
	/// into represented value -> the value is returned,
	/// otherwise default will be used.
	pub fn get(self) -> T {
		match stdenv::var(self.key){
			Ok(value) => {
				value.parse().unwrap_or(self.value)
			},
			Err(_) => {
				self.value
			}
		}
	}
}

#[cfg(test)]
mod unit_tests {
	use crate::utils::Env;
	use crate::utils::tests::WithEnv;

	#[test]
	fn test_env_from_string_default(){
		let e = Env::with("test", String::from("default"));
		assert_eq!(String::from("default"), e.get());
	}

	#[test]
	fn test_env_from_string_env(){
		WithEnv::set("TEST_VALUE", "just a string value").run(||{
			assert_eq!(String::from("just a string value"),
								 Env::with("TEST_VALUE", String::from("default")).get());
		});
	}

	#[test]
	fn test_env_from_i32(){
		WithEnv::set("TEST_VALUE", "1337").run(||{
			let e = Env::with("TEST_VALUE", 1337);
			assert_eq!(1337, e.get());
		});
	}

	#[test]
	fn test_env_from_i32_chain(){
		WithEnv::set("TEST_VALUE", "1337").run(||{
			assert_eq!(1337, Env::with("TEST_VALUE", 1337i32).get());
		});
	}
}