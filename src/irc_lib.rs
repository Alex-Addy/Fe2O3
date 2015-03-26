
pub fn is_valid_channel_name(name: &str) -> bool {
    // channel name definition defined in RFC2812
    // https://tools.ietf.org/html/rfc2812#section-1.3
    name.len() <= 50 &&
    name.starts_with(|c| { match c {
        '&' | '#' | '+' | '!' => true,
        _ => false,
    }}) &&
    // forbidden characters
    !name.contains(|c| { match c {
        ' ' | '\x07' | ',' => true,
        _ => false
    }})
}

pub struct Line(pub String);

impl Line {
    pub fn parse_msg(&self) -> Message {
        let ref line = self.0;

        let prefix_end = match line.starts_with(":") {
            true  => line.find(" ").unwrap(),
            false => 0,
        };
        let pre = match prefix_end {
            0 => None,
            i => Some(&line[1..i]),
        };

        let command_end = match prefix_end {
            0 => line.find(" ").unwrap(),
            i => line[i+1..].find(" ").unwrap_or(line.len())
                + prefix_end // fix offset caused by slicing
                + 1, // move location past space
        };
        let com = match prefix_end {
            0 => &line[..command_end],
            i => &line[i+1..command_end],
        };

        let trailing_start = match line.find(" :") {
            Some(i) => i,
            None    => line.len(),
        };

        let mut par: Vec<&str> = if command_end == trailing_start {
            Vec::new()
        } else {
            line[command_end+1..trailing_start].split(" ").collect()
        };

        if trailing_start != line.len() {
            par.push(&line[trailing_start+2..]);
        };

        Message {
            prefix: pre,
            command: com,
            params: par,
        }
    }
}

pub struct Message<'a> {
    pub prefix: Option<&'a str>,
    pub command: &'a str,
    pub params: Vec<&'a str>,
}

impl<'a> ToString for Message<'a> {
    fn to_string(&self) -> String {
        let mut result = match self.prefix {
            Some(m) => m.to_string(),
            None => "".to_string(),
        };
        // seperating space
        if self.prefix.is_some() { result.push_str(" "); }

        result.push_str(self.command);

        for i in (0..self.params.len()) {
            let cur = self.params[i];
            if cur.contains(" ") {
                if i == self.params.len() - 1 {
                    // trailing parameter
                    result.push_str(" :");
                }
                else {
                    panic!("Invalid message structure: non-trailing param with spaces. M.pre: {:?}, M.com: {:?}, M.par: {:?}", self.prefix, self.command, self.params);
                }
            }
            else {
                result.push_str(" ");
            }
            result.push_str(cur);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn ping_test() {
        let l = super::Line("PING :irc.blab.net".to_string());
        let m = l.parse_msg();

        assert!(m.prefix.is_none());
        assert_eq!(m.command, "PING");
        assert_eq!(m.params.len(), 1);
        assert_eq!(m.params[0], "irc.blab.net");
    }

    #[test]
    fn motd_end_test() {
        let l = super::Line(":irc.blab.net 376 Fe2O3 :End of /MOTD command.".to_string());
        let m = l.parse_msg();

        assert_eq!(m.prefix, Some("irc.blab.net"));
        assert_eq!(m.command, "376");
        assert_eq!(m.params.len(), 2);
        assert_eq!(m.params[0], "Fe2O3");
        assert_eq!(m.params[1], "End of /MOTD command.");
    }

    #[test]
    fn channel_join_test() {
        let l = super::Line(":Fe2O3!~Fe2O3@45.55.151.164 JOIN #tutbot-testing".to_string());
        let m = l.parse_msg();

        assert_eq!(m.prefix, Some("Fe2O3!~Fe2O3@45.55.151.164"));
        assert_eq!(m.command, "JOIN");
        assert_eq!(m.params.len(), 1);
        assert_eq!(m.params[0], "#tutbot-testing");
    }
}
