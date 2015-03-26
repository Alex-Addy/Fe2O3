#![feature(unboxed_closures)]
#![feature(tcp)]

use std::net::{lookup_host, TcpStream, SocketAddr};
use std::io::{Result, BufStream, BufRead, Read, Write};

mod irc_lib;
use irc_lib::{Message, Line};

fn ping_module(msg: &Message) -> Vec<String> {
    if msg.command == "PING" {
        let res = format!("PONG :{}", if msg.params.len() != 0 {
            msg.params[0]
        } else {
            ""
        });
        vec![res]
    } else {
        Vec::new()
    }
}

fn make_reply<'a>(recv: &'a Message, reply: &'a str) -> Message<'a> {
    let reciever = match irc_lib::is_valid_channel_name(recv.params[0]) {
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

fn id_module(msg: &Message) -> Vec<String> {
    if msg.command == "PRIVMSG" && msg.params.len() == 2
        && msg.params[1].starts_with("!id ") {
            let arg = msg.params[1].split("!id ").last().unwrap();
            vec![make_reply(msg, arg).to_string()]
    } else {
        vec![]
    }
}

fn start_connection(host: &str, port: u16) -> Result<TcpStream> {
    let mut sockets = try!(lookup_host(host));

    let intermediate: SocketAddr = sockets.find(|item| {
        item.is_ok() && match item.clone().unwrap() {
            SocketAddr::V4(_) => true,
            SocketAddr::V6(_) => false,
        }
    }) // -> Option<Result<SocketAddr>
        .unwrap() // -> Result<SocketAddr>
        .unwrap(); // -> SocketAddr

    let ip = match intermediate {
        SocketAddr::V4(ipv4) => ipv4.ip().clone(),
        SocketAddr::V6(_) => panic!("Can't handle ipv6"),
    };

    let mut stream = try!(TcpStream::connect((ip, port)));
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

    type Subscriber = fn(&Message) -> Vec<String>;
    let modules = vec![ping_module as Subscriber, id_module as Subscriber];
    let mut line = String::new();

    loop {
        let line_length = try!(stream.read_line(&mut line));
        if line_length <= 2 {
            break;
        }
        line.truncate(line_length - 2);
        println!("< {}", line);

        {
            let wrapper = Line(line.clone());
            let msg = wrapper.parse_msg();
            for ind in (0..modules.len()) {
                for response in modules[ind](&msg) {
                    let _ = try!(send_line(&mut stream, response));
                }
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
        Err(e) => println!("{:?}", e),
    }
}
