use plantuml_parser::{IncludesCollections, PlantUmlFileData};

use clap::{Args, Command, Parser};
use regex::Regex;
use std::fs::read_to_string;
use std::io;
use std::path::PathBuf;

#[derive(Parser, Debug)]
/// description for command.
struct Arg {
    #[clap(short, long)]
    /// description for this argument.
    file: PathBuf,
}

#[derive(Debug)]
struct SequenceMessage {
    from: String,
    to: String,
    message: Option<String>,
    message_type: String,
}
fn parse_message_line(line: &str) -> SequenceMessage {
    let re = Regex::new(r"(\w+)\s*(-?(?:\[\#?\w+\])?-?>)\s*(\w+)\s*:\s*(.+)").unwrap();
    let re_no_msg = Regex::new(r"(\w+)\s*(-?(?:\[\#?\w+\])?-?>)\s*(\w+)\s*").unwrap();
    let re_r_2_l = Regex::new(r"(\w+)\s*(<-?(?:\[\#?\w+\])?-?)\s*(\w+)\s*:\s*(.+)").unwrap();
    let re_r_2_l_no_msg = Regex::new(r"(\w+)\s*(<-?(?:\[\#?\w+\])?-?)\s*(\w+)\s*").unwrap();

    let message;
    if let Some(caps) = re.captures(line) {
        message = SequenceMessage {
            from: caps[1].to_string(),
            to: caps[3].to_string(),
            message: Some(caps[4].to_string()),
            message_type: caps[2].to_string(),
        };
        println!("A:{:?}", message);
    } else if let Some(caps) = re_no_msg.captures(line) {
        message = SequenceMessage {
            from: caps[1].to_string(),
            to: caps[3].to_string(),
            message: None,
            message_type: caps[2].to_string(),
        };
        println!("B:{:?}", message);
    } else if let Some(caps) = re_r_2_l.captures(line) {
        message = SequenceMessage {
            from: caps[3].to_string(),
            to: caps[1].to_string(),
            message: Some(caps[4].to_string()),
            message_type: caps[2].to_string(),
        };
        println!("C:{:?}", message);
    } else if let Some(caps) = re_r_2_l_no_msg.captures(line) {
        message = SequenceMessage {
            from: caps[3].to_string(),
            to: caps[1].to_string(),
            message: None,
            message_type: caps[2].to_string(),
        };
        println!("D:{:?}", message);
    } else {
        message = SequenceMessage {
            from: "".to_string(),
            to: "".to_string(),
            message: None,
            message_type: "".to_string(),
        };
    }
    message
}

///
fn parse_message(data: String) -> Vec<SequenceMessage> {
    let mut messages = Vec::new();
    for line in data.split('\n') {
        println!("{}", line);
        let message = parse_message_line(line);
        messages.push(message);
    }
    messages
}

fn main() -> io::Result<()> {
    let args = Arg::parse();
    let data = read_to_string(args.file)?;
    //let data = read_to_string("tests/sample.puml")?;
    let filedata = PlantUmlFileData::parse_from_str(data).unwrap();
    let content = filedata.get(0).unwrap();
    let result = parse_message(content.inner());
    for item in result {
        println!(
            "{},{},{:?},{}",
            item.from, item.to, item.message, item.message_type
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_line_l2r() -> Result<(), String> {
        let line = "aaa -> bbb: ccc";
        let message = parse_message_line(line);

        println!("{},{:?}", line, message);

        assert_eq!("aaa", message.from);
        assert_eq!("bbb", message.to);
        assert_eq!(Some("ccc".to_string()), message.message);
        assert_eq!("->", message.message_type);
        Ok(())
    }

    #[test]
    fn test_parse_line_r2l2() -> Result<(), String> {
        let line = "aaa --> bbb: ccc";
        let message = parse_message_line(line);

        println!("{},{:?}", line, message);

        assert_eq!("aaa", message.from);
        assert_eq!("bbb", message.to);
        assert_eq!(Some("ccc".to_string()), message.message);
        assert_eq!("-->", message.message_type);
        Ok(())
    }
    #[test]
    fn test_parse_line_l2r_withcolor() -> Result<(), String> {
        let line = "aaa -[#red]-> bbb: ccc";
        let message = parse_message_line(line);

        println!("{},{:?}", line, message);

        assert_eq!("aaa", message.from);
        assert_eq!("bbb", message.to);
        assert_eq!(Some("ccc".to_string()), message.message);
        assert_eq!("-[#red]->", message.message_type);
        Ok(())
    }
    #[test]
    fn test_parse_line_l2r_withcolor2() -> Result<(), String> {
        let line = "aaa [#red]-> bbb: ccc";
        let message = parse_message_line(line);

        println!("{},{:?}", line, message);

        assert_eq!("aaa", message.from);
        assert_eq!("bbb", message.to);
        assert_eq!(Some("ccc".to_string()), message.message);
        assert_eq!("[#red]->", message.message_type);
        Ok(())
    }
    #[test]
    fn test_parse_line_l2r_no_msg() -> Result<(), String> {
        let line = "aaa -> bbb";
        let message = parse_message_line(line);

        println!("{},{:?}", line, message);

        assert_eq!("aaa", message.from);
        assert_eq!("bbb", message.to);
        assert_eq!(None, message.message);
        assert_eq!("->", message.message_type);
        Ok(())
    }
    #[test]
    fn test_parse_line_l2r_no_msg_2() -> Result<(), String> {
        let line = "aaa --> bbb";
        let message = parse_message_line(line);

        println!("{},{:?}", line, message);

        assert_eq!("aaa", message.from);
        assert_eq!("bbb", message.to);
        assert_eq!(None, message.message);
        assert_eq!("-->", message.message_type);
        Ok(())
    }
    #[test]
    fn test_parse_line_l2r_withcolor_no_msg() -> Result<(), String> {
        let line = "aaa -[#red]-> bbb";
        let message = parse_message_line(line);

        println!("{},{:?}", line, message);

        assert_eq!("aaa", message.from);
        assert_eq!("bbb", message.to);
        assert_eq!(None, message.message);
        assert_eq!("-[#red]->", message.message_type);
        Ok(())
    }
    #[test]
    fn test_parse_line_l2r_withcolor2_no_msg() -> Result<(), String> {
        let line = "aaa -[#0000FF]-> bbb";
        let message = parse_message_line(line);

        println!("{},{:?}", line, message);

        assert_eq!("aaa", message.from);
        assert_eq!("bbb", message.to);
        assert_eq!(None, message.message);
        assert_eq!("-[#0000FF]->", message.message_type);
        Ok(())
    }

    #[test]
    fn test_parse_line_r2l() -> Result<(), String> {
        let line = "aaa <- bbb: ccc";
        let message = parse_message_line(line);

        println!("{},{:?}", line, message);

        assert_eq!("bbb", message.from);
        assert_eq!("aaa", message.to);
        assert_eq!(Some("ccc".to_string()), message.message);
        assert_eq!("<-", message.message_type);
        Ok(())
    }

    #[test]
    fn test_parse_line_r2l_no_msg() -> Result<(), String> {
        let line = "aaa <- bbb";
        let message = parse_message_line(line);

        println!("{},{:?}", line, message);

        assert_eq!("bbb", message.from);
        assert_eq!("aaa", message.to);
        assert_eq!(None, message.message);
        assert_eq!("<-", message.message_type);
        Ok(())
    }
    #[test]
    fn test_parse_line_r2l_no_msg2() -> Result<(), String> {
        let line = "aaa <-- bbb";
        let message = parse_message_line(line);

        println!("{},{:?}", line, message);

        assert_eq!("bbb", message.from);
        assert_eq!("aaa", message.to);
        assert_eq!(None, message.message);
        assert_eq!("<--", message.message_type);
        Ok(())
    }

    #[test]
    fn test_parse_line_r2l_withcolor_no_msg() -> Result<(), String> {
        let line = "aaa <-[#red]- bbb";
        let message = parse_message_line(line);

        println!("{},{:?}", line, message);

        assert_eq!("bbb", message.from);
        assert_eq!("aaa", message.to);
        assert_eq!(None, message.message);
        assert_eq!("<-[#red]-", message.message_type);
        Ok(())
    }

    #[test]
    fn test_parse_line_r2l_withcolor_no_msg_2() -> Result<(), String> {
        let line = "aaa <-[#red] bbb";
        let message = parse_message_line(line);

        println!("{},{:?}", line, message);

        assert_eq!("bbb", message.from);
        assert_eq!("aaa", message.to);
        assert_eq!(None, message.message);
        assert_eq!("<-[#red]", message.message_type);
        Ok(())
    }
}
