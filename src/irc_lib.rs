
pub struct Line(String);

impl Line {
    pub fn parse_msg(&self) -> Message {
        let ref line = self.0;
        let prefix_i = match line.starts_with(":") {
            true  => line.find(" ").unwrap(),
            false => 0,
        };

        let command_i = match prefix_i {
            0 => line.find(" ").unwrap(),
            _ => line[prefix_i+1..].find(" ").unwrap(),
        };

        let params_i = match line[command_i..].find(":") {
            Some(i) => i,
            None => line.len(),
        };

        let pre = if prefix_i > 0 {
            Some(&line[..prefix_i])
        } else {
            None
        };

        let com: &str = &line[prefix_i+1..command_i];

        println!("preI: {}, comI: {}, parI: {}, pre: {:?}, com: {}", prefix_i, command_i, params_i, pre, com);
        let par: Vec<&str> = line[command_i+1..params_i].split(" ").collect();
        println!("after par");

        let tra = if params_i != line.len() {
            Some(&line[params_i+1..])
        } else {
            None
        };

        Message {
            prefix: pre,
            command: com,
            params: par,
            trailing: tra,
        }
    }
}

pub struct Message<'a> {
    pub prefix: Option<&'a str>,
    pub command: &'a str,
    pub params: Vec<&'a str>,
    pub trailing: Option<&'a str>,
}

#[cfg(test)]
mod tests {
    #[test]
    fn ping_test() {
        let l = super::Line("PING :irc.blab.net".to_string());
        let m = l.parse_msg();

        //assert!(m.prefix.is_none());
        //assert_eq!(m.command, "PING");
        //assert_eq!(m.params, Vec::<&str>::new());
        //assert_eq!(m.trailing, Some("irc.blab.net"));
    }
}
