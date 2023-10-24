mod objects;
mod services;
mod demo;

pub struct Client {
	api_url: String,
	token: String,

	inlines: Option<services::Inline>,
}