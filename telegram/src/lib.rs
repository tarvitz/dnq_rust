mod objects;
mod services;

pub struct Client {
	api_url: String,
	token: String,

	inlines: Option<services::Inline>,
}