use std::io::prelude::*;
use std::{io::Read};
use std::net::{TcpListener, TcpStream};
use log::{debug, info}; // trace, warn

use dnq::ThreadPool;

static GET:&[u8; 16] = b"GET / HTTP/1.1\r\n";
static HTTP_200_OK:&str = "HTTP/1.1 200 OK\r\n\r\n";
static HTTP_400_BAD_REQUEST:&str = "HTTP/1.1 400 BAD REQUEST\r\n\r\n";

fn default_handler(mut stream: TcpStream) {
	info!("requested");
	// let mut buf = String::new();
	// stream.read_to_string(&mut buf).unwrap();
	let mut buf = [0; 512]; // replace with bigger buffer and better reading
	stream.read(&mut buf).unwrap();

	let (status_line, contents) = if buf.starts_with(GET){
		(HTTP_200_OK, "ok")
	} else {
		(HTTP_400_BAD_REQUEST, "")
	};
	debug!("received: {:?}", &buf);

	let response = format!("{}{}", status_line, contents);
	stream.write(response.as_bytes()).unwrap();
	stream.flush().unwrap();
}

fn main(){
	env_logger::init();

	info!("starting a webserver");

	let pool = ThreadPool::new(8);

	let listener = TcpListener::bind("0.0.0.0:8443").unwrap();
	for stream in listener.incoming(){
		let stream = stream.unwrap();
		pool.execute(||{
			debug!("executing");
			default_handler(stream)
		})
	}

	info!("exiting.")
}