#![feature(io)]
#![feature(old_io)]

use std::old_io::net::{tcp, addrinfo};
use std::old_io::{IoResult, BufferedStream, Stream};

fn start_connection(host: &str, port: u16) -> IoResult<tcp::TcpStream> {
	let res = try!(addrinfo::get_host_addresses(host));
	let mut stream = try!(tcp::TcpStream::connect((res[0], port)));
	try!(stream.set_nodelay(true));

	return Ok(stream);
}

fn send_line_fmt<T: Writer>(sink: &mut T, fmt: std::fmt::Arguments) -> IoResult<()> {
	sink.write_fmt(fmt).and(
	sink.flush())
}

/// Spins on stream, acting as the main control loop
fn listen<S: Stream>(mut stream: BufferedStream<S>) {
	println!("Starting to listen");
	let mut result = stream.read_line();
	while result.is_ok() {
		let line = result.unwrap();
		print!("{}", line);

		result = stream.read_line();
	}
}

fn main() {
	let server = "irc.freenode.org";
	let port   = 6667;
	let chan = "#tutbot-testing";
	let nick = "tutbot";

	let mut stream = BufferedStream::new(start_connection(server, port).unwrap());

	send_line_fmt(&mut stream, format_args!("{} {}\r\n", "NICK", nick)).unwrap();
	send_line_fmt(&mut stream, format_args!("{} {}{}\r\n","USER", nick," 0 * :tutorial bot")).unwrap();
	send_line_fmt(&mut stream, format_args!("{} {}\r\n", "JOIN", chan)).unwrap();

	listen(stream);
}
