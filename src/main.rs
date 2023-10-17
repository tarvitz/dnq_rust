use dnq;

use actix_web::{App, HttpServer};
use log::{info};

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

	HttpServer::new(||{
		App::new()
			.service(dnq::endpoints::hello)
			.service(dnq::endpoints::readyz)
	})
		.workers(8)
		.bind(config.address.as_str())?
		.run()
		.await
}