
use utils::{Message, make_reply};

extern crate rand;
use self::rand::Rng;
use std::str::FromStr;
pub fn random_module(msg: &Message) -> Vec<String> {
    if msg.command == "PRIVMSG" && msg.params.len() == 2
        && msg.params[1].starts_with("!rand") {
            let mut rng = rand::thread_rng();
            let args: Vec<&str> = msg.params[1].split(" ").collect();

            let bounds = if args.len() == 2 {
                (Ok(0), isize::from_str(args[1]))
            } else if args.len() == 3 { // both bounds
                (isize::from_str(args[1]), isize::from_str(args[2]))
            } else {
                (Ok(0), Ok(100))
            };

            let num = match bounds {
                    (Err(e), _) => format!("Failed to parse first bound with err {:?}", e),
                    (_, Err(e)) => format!("Failed to parse second bound with err {:?}", e),
                    (Ok(b1), Ok(b2)) if b1 == b2 =>
                            format!("Can't generate random in empty range [{}, {})", b1, b2),
                    (Ok(b1), Ok(b2)) if b1 > b2 => rng.gen_range::<isize>(b2, b1).to_string(),
                    (Ok(b1), Ok(b2)) => rng.gen_range::<isize>(b1, b2).to_string(),
            };

            vec![make_reply(msg, num.as_ref()).to_string()]
    } else {
        vec![]
    }
}
