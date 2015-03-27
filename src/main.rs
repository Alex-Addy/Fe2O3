#![feature(unboxed_closures)]
#![feature(convert)]
#![feature(tcp)]
#![feature(lookup_host)]

mod utils;
use utils::{Message, Subscriber};

mod modules;

fn main() {
    let server = "concrete.slashnet.org";
    let port   = 6667;
    let chan = "#uakroncs";
    let nick = "Fe2O3";

    let modules:Vec<Subscriber> = vec![
                        modules::ping,
                        modules::id,
                        modules::random,
                        modules::fortune];

    match utils::connect_and_listen(server, port, nick, vec![chan], modules) {
        Ok(()) => (),
        Err(e) => println!("{:?}", e),
    }
}
