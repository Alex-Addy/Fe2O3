
use utils::{Message, make_reply};
use std::process::Command;

pub fn fortune_module(msg: &Message) -> Vec<String> {
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
