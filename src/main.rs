#![allow(non_snake_case)]

extern crate dns_lookup;

mod utils;
use utils::Subscriber;

mod modules;

fn main() {
    let server = "concrete.slashnet.org";
    let port   = 6667;
    let chan = "#uakroncs";
    let nick = "Fe2O3";

    // when linking markov in, construct it
    // let model = MarkovModel::new(corpus)
    // then wrap the relevant function in a closure
    // to satisfy the Subscriber type
    // let markov_message =
    //    | msg: Message | model.markov_module(msg)

    let modules:Vec<Subscriber> = vec![
                        modules::ping,
                        modules::id,
                        modules::random,
                        modules::fortune];

    match utils::connect_and_listen(server, port, nick, vec![chan], modules) {
        Ok(()) => (),
        Err(e) => println!("Connection ended with error: {:?}", e),
    }
}
