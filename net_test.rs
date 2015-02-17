#![feature(net)]
#![feature(io)]
#![feature(core)]
use std::net;
use std::io::Read;


fn main() {
	let server = "irc.freenode.org";
	let port   = 6667;
	let res = net::lookup_host(server);

	let socket = net::SocketAddr::new(res.unwrap().next().unwrap().unwrap().ip(), port);
	let mut stream = net::TcpStream::connect(&socket).unwrap();
	stream.set_nodelay(true);

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
