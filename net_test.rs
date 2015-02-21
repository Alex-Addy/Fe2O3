#![feature(io)]

use std::old_io::net::{tcp, addrinfo};
use std::old_io::{IoResult, BufferedStream};

fn start_connection(host: &str, port: u16) -> IoResult<tcp::TcpStream> {
	let res = try!(addrinfo::get_host_addresses(host));
	let mut stream = try!(tcp::TcpStream::connect((res[0], port)));
	try!(stream.set_nodelay(true));

	return Ok(stream);
}

fn main() {
	let server = "irc.freenode.org";
	let port   = 6667;
	let chan = "#tutbot-testing";
	let nick = "tutbot";

	let mut stream = BufferedStream::new(start_connection(server, port).unwrap());

	stream.write_fmt(format_args!("{} {}\r\n", "NICK", nick)).unwrap();
	stream.flush().unwrap();
	stream.write_fmt(format_args!("{} {}{}\r\n", "USER", nick, " 0 * :tutorial bot")).unwrap();
	stream.flush().unwrap();
	stream.write_fmt(format_args!("{} {}\r\n", "JOIN", chan)).unwrap();
	stream.flush().unwrap();

	let mut result = stream.read_line();
	while result.is_ok() {
		print!("{}", result.unwrap());
		result = stream.read_line();
	}
}
