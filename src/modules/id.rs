
use utils::{Message, make_reply};

pub fn id_module(msg: &Message) -> Vec<String> {
    if msg.command == "PRIVMSG" && msg.params.len() == 2
        && msg.params[1].starts_with("!id ") {
            let arg = msg.params[1].split("!id ").last().unwrap();
            vec![make_reply(msg, arg).to_string()]
    } else {
        vec![]
    }
}
