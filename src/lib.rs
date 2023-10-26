pub mod endpoints;
pub mod utils;

use std::fs;
use crate::utils::env::Env as E;

static DEFAULT_ADDR: &str = "0.0.0.0:8443";

use serde::Deserialize;
use stringreader::StringReader;
use telegram::objects::{Quote, set_quotes};


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

pub enum Error {
	CantLoadQuotes(String),
}

pub struct Config {
	pub token: String,
	pub address: String,
	pub workers: u8,
	pub quotes: String,
}

impl Config {
	pub fn from_env() -> Config {
		Config {
			token: E::with("DNQ_TOKEN", String::from("")).get(),
			address: E::with("DNQ_ADDRESS", String::from(DEFAULT_ADDR)).get(),
			workers: E::with("DNQ_WORKERS", 8).get(),
			quotes: E::with("DNQ_QUOTES", String::from("config.yaml")).get(),
		}
	}
}

#[derive(Debug, Deserialize)]
struct Quotes {
	quotes: Vec<Quote>
}

pub fn load_quotes(cfg: &Config) -> Result<u16, Error> {
	let result = fs::read_to_string(cfg.quotes.as_str());

	return match result {
		Ok(contents) => {
			let contents = StringReader::new(contents.as_str());
			let result: Result<Quotes, _> = serde_yaml::from_reader(contents);

			return match result {
				Ok(container) => {
					let size = container.quotes.len() as u16;
					set_quotes(&container.quotes);

					Ok(size)
				},
				_ => Err(Error::CantLoadQuotes(format!("can deserialize"))),
			}
		},
		Err(e) => Err(Error::CantLoadQuotes(format!("can't open file: {:?}", e))),
	};
}
