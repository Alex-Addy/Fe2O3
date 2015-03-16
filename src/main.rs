#![feature(io)]
#![feature(net)]

use std::net::{lookup_host, TcpStream};
use std::io::{Result, BufStream, BufRead, Read, Write};

fn start_connection(host: &str, port: u16) -> Result<TcpStream> {
	let mut res = try!(lookup_host(host));
	let mut stream = try!(TcpStream::connect(&(res.next()
                    .expect("Failed to get ip address for host")
                    .unwrap().ip(), port)));
	try!(stream.set_nodelay(true));
    try!(stream.set_keepalive(Some(30)));

	return Ok(stream);
}

fn send_line<T: Write>(sink: &mut T, line: String) -> Result<()> {
    let line_r_n: String = line.clone()  + "\r\n";
    let bytes: &[u8] = line_r_n.as_bytes();
    print!("> {}", line);
	sink.write(bytes).and(
	sink.flush())
}

/// Spins on stream, acting as the main control loop
fn listen<S: Read + Write>(mut stream: BufStream<S>) {
	println!("Starting to listen");
    let mut line = String::new();
	let mut result = stream.read_line(&mut line);
	while result.is_ok() && result.unwrap() > 0 {
		print!("< {}", line);

        line.truncate(0);
		result = stream.read_line(&mut line);
	}
}

fn main() {
	let server = "irc.freenode.org";
	let port   = 6667;
	let chan = "#tutbot-testing";
	let nick = "Fe2O3";

	let mut stream = BufStream::new(start_connection(server, port).unwrap());

	send_line(&mut stream, format!("{} {}\r\n", "NICK", nick)).unwrap();
	send_line(&mut stream, format!("{} {}{}\r\n","USER", nick," 0 * :tutorial bot")).unwrap();
	send_line(&mut stream, format!("{} {}\r\n", "JOIN", chan)).unwrap();

	listen(stream);
}
