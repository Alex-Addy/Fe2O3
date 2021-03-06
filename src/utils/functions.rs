use std::net::TcpStream;
use std::io::{Result, Write, BufRead, BufReader};

use dns_lookup::lookup_host;

use super::structs::{Message, Line};
use super::Subscriber;

fn send_line<T: Write>(sink: &mut T, line: String) -> Result<()> {
    let line_r_n: String = line + "\r\n";
    let bytes: &[u8] = line_r_n.as_bytes();
    print!("> {}", line_r_n);
	sink.write(bytes)
    .and(sink.flush())
}

pub fn make_reply<'a>(recv: &'a Message, reply: &'a str) -> Message<'a> {
    let reciever = match is_valid_channel_name(recv.params[0]) {
        true  => recv.params[0].clone(), //ASSUMPTION: a valid channel name is always a channel
        false => recv.prefix.unwrap().split("!").next().unwrap().clone(),
        // prefix = servername / ( nickname [ [ "!" user ] ] "@" host ] )
    };
    Message {
        prefix: None,
        command: recv.command.clone(),
        params: vec![reciever, reply],
    }
}

pub fn is_valid_channel_name(name: &str) -> bool {
    // channel name definition defined in RFC2812
    // https://tools.ietf.org/html/rfc2812#section-1.3
    name.len() <= 50 &&
    name.starts_with(|c| { match c {
        '&' | '#' | '+' | '!' => true,
        _ => false,
    }}) &&
    // forbidden characters
    !name.contains(|c| { match c {
        ' ' | '\x07' | ',' => true,
        _ => false
    }})
}

pub fn connect_and_listen(server: &str, port: u16, nick: &str, channels: Vec<&str>,
                          modules: Vec<Subscriber>) -> Result<()> {
    let mut stream = try!(start_connection(server, port));
    let mut reader = BufReader::new(try!(stream.try_clone()));

    try!(send_line(&mut stream, format!("NICK {}", nick)));
    try!(send_line(&mut stream, format!("USER {}{}", nick, " 0 * :rust bot")));

    let mut line = String::new();
    loop {
        let line_length = try!(reader.read_line(&mut line));
        if line_length <= 2 {
            panic!("unexpected line too short when setting up");
        }
        line.truncate(line_length - 2);
        println!("< {}", line);

        {
            let wrapper = Line(line.clone());
            let msg = wrapper.parse_msg();

            if msg.command == "PING" {
                let _ = try!(send_line(&mut stream, line.replace("PING", "PONG")));
            }

            if msg.command == "376" { // End of MOTD
                for chan in channels {
                    // TODO deal with invalid channel name
                    let _ = try!(send_line(&mut stream, format!("JOIN {}", chan)));
                }
                break;
            }
        }

        line.clear();
    }

    listen(reader, stream, modules)
}

// start_connection will attempt to lookup host and attempt to open a socket
fn start_connection(host: &str, port: u16) -> Result<TcpStream> {
    let sockets = try!(lookup_host(host));

    let stream = try!(TcpStream::connect((sockets[0], port)));

    // turn off Nagle so that lines sent are not buffered
    try!(stream.set_nodelay(true));

    return Ok(stream);
}

/// Spins on stream,
/// feeding all lines to each module and writing returned lines back to the stream
fn listen<R: BufRead, W: Write>(mut source: R, mut sink: W, modules: Vec<Subscriber>) -> Result<()> {
    println!("Starting to listen");

    // string for input lines to be read into
    // must be cleared after each line is handled
    let mut line = String::new();

    loop {
        let result = source.read_line(&mut line);
        if result.is_err() {
            continue
        }

        let line_length = result.unwrap();
        if line_length <= 2 {
            break;
        }
        line.truncate(line_length - 2);
        println!("< {}", line);

        {
            let wrapper = Line(line.clone());
            let msg = wrapper.parse_msg();
            for ind in 0..modules.len() {
                for response in modules[ind](&msg) {
                    let _ = try!(send_line(&mut sink, response));
                }
            }
        }

        line.clear();
    }

    Ok(())
}
