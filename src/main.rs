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
    //try!(stream.set_keepalive(Some(30)));

    return Ok(stream);
}

fn send_line<T: Write>(sink: &mut T, line: String) -> Result<()> {
    let line_r_n: String = line + "\r\n";
    let bytes: &[u8] = line_r_n.as_bytes();
    print!("> {}", line_r_n);
	sink.write(bytes)
    .and(sink.flush())
}

/// Spins on stream, acting as the main control loop
fn listen<S: Read + Write>(mut stream: BufStream<S>) -> Result<()> {
    println!("Starting to listen");

    let mut line = String::new();
    loop {
        let line_length = try!(stream.read_line(&mut line));
        if line_length <= 2 {
            break;
        }
        line.truncate(line_length - 2);
        println!("< {}", line);

        {
            let bytes = line.as_bytes();
            if bytes.starts_with("PING :".as_bytes()) && bytes.len() > 6{
                send_line(&mut stream, format!("PONG :{}",
                                           String::from_utf8(bytes[6..].to_vec()).unwrap()
                                          ));
            }
        }

        line.clear();
    }

    Ok(())
}

fn main() {
    let server = "irc.freenode.org";
    let port   = 6667;
    let chan = "#tutbot-testing";
    let nick = "Fe2O3";

    let mut stream = BufStream::new(start_connection(server, port).unwrap());

    send_line(&mut stream, format!("{} {}", "NICK", nick)).unwrap();
    send_line(&mut stream, format!("{} {}{}","USER", nick," 0 * :tutorial bot")).unwrap();
    send_line(&mut stream, format!("{} {}", "JOIN", chan)).unwrap();

    match listen(stream) {
        Ok(()) => (),
        Err(e) => println!("Error: {}", e.description()),
    }
}
