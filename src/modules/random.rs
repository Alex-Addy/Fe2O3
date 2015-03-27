
use utils::{Message, make_reply};

extern crate rand;
use self::rand::Rng;
use std::str::FromStr;
pub fn random_module(msg: &Message) -> Vec<String> {
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
