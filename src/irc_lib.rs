
pub struct Message<'a> {
    pub prefix: Option<&'a str>,
    pub command: &'a str,
    pub params: Vec<&'a str>,
    pub trailing: Option<&'a str>,

    line: String,
}

impl<'a> Message<'a> {
    pub fn new(line: String) -> Message<'a> {
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

        let par: Vec<&str> = line[command_i+1..params_i].split(" ").collect();

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

            line: line,
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn ping_test() {
        let m = super::Message::new("PING :irc.blab.net".to_string());

        assert_eq!(m.prefix, None);
        assert_eq!(m.command, "PING");
        assert_eq!(m.params, Vec::<&str>::new());
        assert_eq!(m.trailing, Some("irc.blab.net"));
    }
}
