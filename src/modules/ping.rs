
use utils::{Message, make_reply};

pub fn ping_module(msg: &Message) -> Vec<String> {
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
