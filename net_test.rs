#![feature(net)]
#![feature(io)]
#![feature(core)]
use std::net;

use std::io::Read;
use std::io::Result;
use std::io::Write;

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
	let chan = "#tutbot-testing";
	let nick = "tutbot";

	let mut stream = start_connection(server, port).unwrap();

	stream.write_fmt(format_args!("{} {}\r\n", "NICK", nick));
	stream.write_fmt(format_args!("{} {}{}\r\n", "USER", nick, " 0 * :tutorial bot"));
	stream.write_fmt(format_args!("{} {}\r\n", "JOIN", chan));

	let mut buf = [0; 64];
	let mut bytes_read = stream.read(&mut buf).unwrap();
	while bytes_read > 0 {
		print!("{}", match std::str::from_utf8(&buf[..bytes_read]) {
			Ok(s) => s,
			Err(e) => match e {
				std::str::Utf8Error::InvalidByte(_) => "Invalid Byte",
				std::str::Utf8Error::TooShort => "Too short",
			},
		});
		bytes_read = stream.read(&mut buf).unwrap();
	}
}
