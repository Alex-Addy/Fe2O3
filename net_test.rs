#![feature(net)]
#![feature(io)]
#![feature(core)]
use std::net;
use std::io::Read;
use std::io::Result;

fn start_connection(host: &str, port: u16) -> Result<net::TcpStream> {
	let mut res = try!(net::lookup_host(host));
	let first = try!(res.next().unwrap());

	let socket = net::SocketAddr::new(first.ip(), port);
	let mut stream = try!(net::TcpStream::connect(&socket));
	try!(stream.set_nodelay(true));

	return Ok(stream);
}

fn main() {
	let server = "irc.freenode.org";
	let port   = 6667;
	let mut stream = start_connection(server, port).unwrap();

	let mut buf = [0; 128];
	let mut bytes_read = stream.read(&mut buf).unwrap();
	while bytes_read > 0 {
		print!("{}", match std::str::from_utf8(&buf) {
			Ok(s) => s,
			Err(e) => match e {
				std::str::Utf8Error::InvalidByte(_) => "Invalid Byte",
				std::str::Utf8Error::TooShort => "Too short",
			},
		});
		bytes_read = stream.read(&mut buf).unwrap();
	}
}
