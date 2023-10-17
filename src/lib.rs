pub mod endpoints;
pub mod utils;

use std::env;

static DEFAULT_ADDR: &str = "0.0.0.0:8443";

#[cfg(feature = "openssl")]
use openssl::ssl::{SslAcceptor, SslAcceptorBuilder, SslMethod, SslFiletype};

#[cfg(feature = "openssl")]
pub fn tls () -> SslAcceptorBuilder {
	let mut builder = SslAcceptor::mozilla_intermediate(
		SslMethod::tls()).unwrap();
	builder.set_private_key_file("resources/server.key", SslFiletype::PEM).unwrap();
	builder.set_certificate_chain_file("resources/server.pem").unwrap();

	builder
}

pub struct Config {
	pub address: String,
	pub workers: u32,
}

impl Config {
	pub fn new() -> Config {
		Config{address: String::from("0.0.0.0:8443"), workers: 8}
	}

	pub fn from_env() -> Config {
		Config {
			address: utils::Env::
				with("DNQ_ADDRESS", String::from("0.0.0.0:8443")).get(),
			workers: utils::Env::
				with("DNQ_WORKERS", 8).get(),
		}
	}

	pub fn with_address(mut self, new_address: &str) -> Self {
		self.address = String::from(new_address);
		return self
	}

	pub fn set_address(&mut self, new_address: &str) {
		self.address = String::from(new_address);
	}
}
