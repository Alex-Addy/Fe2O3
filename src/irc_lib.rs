
pub struct Line(String);

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
            i => line[i+1..].find(" ").unwrap_or(line.len()),
        };
        let com = match prefix_end {
            0 => &line[..command_end],
            i => &line[i..command_end],
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
}
