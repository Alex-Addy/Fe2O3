#![feature(unboxed_closures)]
#![feature(tcp)]
#![feature(convert)]

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

extern crate rand;
use rand::Rng;
use std::str::FromStr;
fn random_module(msg: &Message) -> Vec<String> {
    if msg.command == "PRIVMSG" && msg.params.len() == 2
        && msg.params[1].starts_with("!rand") {
            let mut rng = rand::thread_rng();
            let args: Vec<&str> = msg.params[1].split(" ").collect();

            let num = if args.len() == 2 { // bound aroung zero
                let up = isize::from_str(args[1]);
                match up {
                    Err(e) => format!("Bound parsing failed with err {:?}", e),
                    Ok(b) if b == 0 => "Can't generate random in empty range [0, 0)".to_string(),
                    Ok(b) if b < 0 => rng.gen_range::<isize>(b, 0).to_string(),
                    Ok(b) /*b > 0*/ => rng.gen_range::<isize>(0, b).to_string(),
                }
            } else if args.len() == 3 { // both bounds
                match (isize::from_str(args[1]), isize::from_str(args[2])) {
                    (Err(e), _) => format!("Failed to parse first bound with err {:?}", e),
                    (_, Err(e)) => format!("Failed to parse second bound with err {:?}", e),
                    (Ok(b1), Ok(b2)) if b1 == b2 =>
                            format!("Can't generate random in empty range [{}, {})", b1, b2),
                    (Ok(b1), Ok(b2)) if b1 > b2 => rng.gen_range::<isize>(b2, b1).to_string(),
                    (Ok(b1), Ok(b2)) => rng.gen_range::<isize>(b1, b2).to_string(),
                }
            } else {
                rng.gen_range::<isize>(0, 100).to_string()
            };
            vec![make_reply(msg, num.as_ref()).to_string()]
    } else {
        vec![]
    }
}

use std::process::Command;
fn fortune_module(msg: &Message) -> Vec<String> {
    if msg.command == "PRIVMSG" && msg.params.len() == 2
        && msg.params[1].starts_with("!fortune") {
        let output = Command::new("fortune")
            .arg("-a") // use all fortunes, including offensive ones
            .arg("-s") // use only short fortunes
            .output();
        let res = match output {
            Ok(out) => if out.status.success() {
                String::from_utf8(out.stdout).unwrap()
            } else {
                format!("process exited abnormally with status {:?}", out.status.code())
            },
            Err(e)  => format!("Could not execute process: {:?}", e),
        }.replace("\n", " ").replace("\t", " ");
        vec![make_reply(msg, res.as_ref()).to_string()]
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
    let modules = vec![ping_module as Subscriber,
                        id_module as Subscriber,
                        random_module as Subscriber,
                        fortune_module as Subscriber];
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
