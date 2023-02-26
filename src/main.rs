use regex::Regex;
use std::collections::HashMap;
use std::fs;

fn create_regex_dict() -> HashMap<&'static str, Regex> {
    let mut regex_dict = HashMap::new();
    regex_dict.insert(
        "credit_card",
        Regex::new(r"\b(?:\d[ -]*?){13,16}\b").unwrap(),
    );
    regex_dict.insert(
        "email",
        Regex::new(r"\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Z|a-z]{2,}\b").unwrap(),
    );
    regex_dict.insert(
        "url",
        Regex::new(r"\b^[-a-zA-Z0-9@:%._\+~#=]{1,256}\.[a-zA-Z0-9()]{1,6}\b(?:[-a-zA-Z0-9()@:%_\+.~#?&//=]*)$\b").unwrap(),
    );
    regex_dict.insert(
        "ip_address",
        Regex::new(r"\b^(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)$\b").unwrap(),
    );
    regex_dict.insert(
        "srv_dns",
        Regex::new(r"\b((xn--)?[a-z0-9\w]+(-[a-z0-9]+)*\.)+[a-z]{2}\b").unwrap(),
    );
    regex_dict
}

struct Data {
    row: String,
    is_juicy: bool,
    content_type: &'static str,
}

impl Default for Data {
    fn default() -> Self {
        Self {
            row: "0".to_string(),
            content_type: "None",
            is_juicy: false,
        }
    }
}

impl Data {
    fn new(row: String) -> Self {
        Self { 
        	row,
        	..Default::default() 
        }
    }

    fn to_row(&self) -> String {
        self.row.clone()
    }

    fn set_content_type(&mut self, regex_dict: &HashMap<&'static str, Regex>) -> () {
        for (id, regex) in regex_dict {
            if regex.is_match(&self.row) {
            	self.content_type = id;
                self.is_juicy = true;
                return;
            }
        }
        self.is_juicy = false;
    }

}

fn main() -> Result<(), std::io::Error> {
    let regex_dict = create_regex_dict();
    let file_contents = fs::read_to_string("/usr/share/wordlists/seclists/Passwords/xato-net-10-million-passwords.txt")?;
    for line in file_contents.lines() {
        let mut data = Data::new(line.to_string());
        data.set_content_type(&regex_dict);
        if data.is_juicy {
            println!("{}", data.to_row());
        }
    }
    Ok(())
}

