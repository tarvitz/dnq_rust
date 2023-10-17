use std::env as stdenv;
use std::str::FromStr;

/// Env provides a way to fetch a value from
/// environment variables and set a default value on any issue
/// # Example
/// ```
/// use dnq::utils::Env;
/// let e = Env::new("TEST_VALUE", 1337u32);
/// assert_eq!(1337u32, e.env());
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
	use std::env as stdenv;
	use crate::utils::Env;

	#[test]
	fn test_env_from_string_default(){
		let e = Env::with("test", String::from("default"));
		assert_eq!(String::from("default"), e.get());
	}

	#[test]
	fn test_env_from_string_env(){
		let e = Env::with("test", String::from("default"));
		assert_eq!(String::from("default"), e.get());
	}

	#[test]
	fn test_env_from_i32(){
		let env_var:&str = "TEST_VALUE";
		stdenv::set_var(&env_var, "1337");
		let e = Env::with(&env_var, 1337);
		stdenv::remove_var(&env_var);
		assert_eq!(1337, e.get());
	}

	#[test]
	fn test_env_from_i32_chain(){
		let env_var:&str = "TEST_VALUE";
		stdenv::set_var(&env_var, "1337");
		assert_eq!(1337, Env::with("TEST_VALUE", 1337i32).get());
		stdenv::remove_var(&env_var);
	}
}