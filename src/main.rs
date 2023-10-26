use dnq;
use actix_web::{App, HttpServer};
use log::{info, error};

/*
#[cfg(feature = "openssl")]
	{
		let builder = tls();
		server.bind_openssl(address.as_str(), builder)?;
	}
*/

#[actix_web::main]
async fn main() -> std::io::Result<()>{
	env_logger::init();

	let config = dnq::Config::from_env();

	info!("starting a webserver at {}", config.address.as_str());

	let loaded = dnq::load_quotes(&config);
	match loaded {
		Ok(amount) => info!("loading quotes: {}", amount),
		Err(_) => error!("issue on initializing"),
	};

	HttpServer::new(||{
		App::new()
			.service(dnq::endpoints::hello)
			.service(dnq::endpoints::readyz)
			.service(dnq::endpoints::answer)
	})
		.workers(config.workers as usize)
		.bind(config.address.as_str())?
		.run()
		.await
}