use std::io;
use regex::Regex;
use std::fs::File;
use clap::{Arg, App};
use std::collections::HashMap;
use std::io::{BufRead, BufReader};
use std::time::{Instant};

/// Data structure for a raw_line of text data
struct Data {
    raw_line: String,
    is_juicy: bool,
    content_type: &'static str,
    exact: String,
}

impl Default for Data {
    fn default() -> Self {
        Self {
            raw_line: "0".to_string(),
            content_type: "None",
            is_juicy: false,
            exact: "None".to_string(),
        }
    }
}

impl Data {
    fn new(raw_line: String) -> Self {
        Self { 
        	raw_line,
        	..Default::default() 
        }
    }

    fn to_message(&self) -> String {
        /*
        Forms a messages from the content type and the text
        */
    	format!("{}: {}", self.content_type, self.raw_line)

    }

    fn to_exact_message(&self) -> String {
        format!("{}: {}", self.content_type, self.exact)
    }

    
    // fn to_row(&self) -> String {
    //     /*
    //     Converts the line to a CSV row
    //     */
    //     format!("{},{}", self.content_type, self.raw_line)
    // }

    fn set_content_type(&mut self, regex_map: &HashMap<&'static str, Regex>) -> () {
        /* Searches through the specified regexes to determine if the data 
        provided is valuable information for the provided user
        :param regex_map: Created regexes to search through
        */
        for (content_type, regex) in regex_map.iter() {
            if let Some(capture) = regex.captures(&self.raw_line) {
                if let Some(file_name) = capture.get(1) {
                    self.exact = file_name.as_str().to_owned();
                    self.content_type = content_type;
                    self.is_juicy = true;
                    break;
                }
            }
        }
    }
}

struct DataSurgeon {
    matches: clap::ArgMatches,
}


impl  DataSurgeon {

    fn build_regex_query(&self) -> HashMap<&'static str, Regex>{
        /* Builds a regex query to search for important information 
        :return: A HashMap containg the content type and the regex associated 

        Hello, Contributers! 
        To add a new regex, add a new raw_line to the following line. 
        The key is the name of the content you are searching for, 
        and the value is the associated regex.

        please note how the regexes use \b syntax

        let regex_map: HashMap<&str, Regex> = [
                ("test_regex", Regex::new(r"\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Z|a-z]{2,}\b").unwrap()),
            ].iter().cloned().collect();

        The key is also used to display to the user what was found, so make it clear and concise, e.g., "email_address: Matched content."
        Note that the regex patterns must conform to Rust's regex syntax. You can test your regex patterns at https://regexr.com/.
        */
        let regex_map: HashMap<&str, Regex> = [
                ("email", Regex::new(r"\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Z|a-z]{2,}\b").unwrap()),
                ("url", Regex::new(r"(\w+://\S+)").unwrap()),
                ("ip_address", Regex::new(r"\b((25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?).){3}(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\b").unwrap()),
                ("ipv6_address", Regex::new(r"([0-9a-fA-F]{1,4}(:[0-9a-fA-F]{1,4}){7})").unwrap()),
                ("srv_dns", Regex::new(r"\b((xn--)?[a-z0-9\w]+(-[a-z0-9]+)*\.)+[a-z]{2}\b").unwrap()),
                ("files", Regex::new(r"([\w,\s-]+\.(txt|pdf|doc|docx|xls|xlsx|xml|jpg|jpeg|png|gif|bmp|csv|json|yaml|log|tar|tgz|gz|zip|rar|7z|exe|dll|bat|ps1|sh|py|rb|js|mdb|sql|db|dbf|ini|cfg|conf|bak|old|backup|pgp|gpg|aes|dll|sys|drv|ocx|pcap|tcpdump))").unwrap()),
                ("mac_address", Regex::new(r"([0-9a-fA-F]{2}(:[0-9a-fA-F]{2}){5})").unwrap()),
                ("domain_users", Regex::new(r"\b^[A-Za-z0-9._%+-]+(?:@[A-Za-z0-9._%+-]+)?@(?:.+\.)?(?:lan|local\.(?:home|company|corp)|workgroup|mshome\.net|domain(?:\.local)?)\.[A-Za-z]{2,4}$\b").unwrap()),
                ("credit_card", Regex::new(r"\b(?:\d[ -]?){15,16}\d\b").unwrap()),
            ].iter().cloned().collect();
        let keys: Vec<&str> = regex_map.keys().copied().collect();
        /*
        If the user didn't specify any extraction choices (e.g: email, url, ip_address)
        */
        if keys.iter().all(|value_name| !self.matches.is_present(value_name)) {
            return regex_map;
        }
        /*
        If they did, then remove the ones they didnt select
        */
        let filtered_map: HashMap<&str, Regex> = keys
            .into_iter()
            .filter(|&key| {
                let has_match = self.matches.is_present(key); 
                let is_empty = regex_map[key].as_str().is_empty();
                has_match && !is_empty

            })
            .map(|key| (key, regex_map[key].clone()))
            .collect();
        filtered_map
    }


    fn handle(&self, line: &std::io::Result<String>, regex_map: &HashMap<&'static str, Regex>) {
        /* Handles a line of text and applies various regexes to determine if the 
        content is important
        :param line: Line to process
        :param regex_map: Regexes to apply
        */
        let line = match line {
            Ok(line) => line,
            Err(_) => {
                return;
            }
        };
        if line.is_empty() {
            return;
        }
        let mut data: Data = Data::new(line.to_string());
        data.set_content_type(regex_map);
        if data.is_juicy {
            if self.matches.is_present("output") && !self.matches.value_of("output").unwrap_or_default().is_empty() {

                return;
            } 
            if self.matches.is_present("junk") {
                println!("{}", data.to_exact_message());
            } else {
                println!("{}", data.to_message());
            }
        }
    }

    fn iterate_file(&self, path: &str) {
        /* Iterates through the specified file to find important information
        :param path: file to process
        */
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);
        let regex_map = self.build_regex_query();
        for line in reader.lines() {
            self.handle(&line, &regex_map);
        }

    }

    fn iterate_stdin(&self) {
        /* Iterates through the standard output to find important informatio
        :param path: file to process
        */
        let stdin = io::stdin();
        let reader = stdin.lock();
        let regex_map = self.build_regex_query();
        for line in reader.lines() {
            self.handle(&line, &regex_map);
        }

    }

    fn process(&mut self) {
        /* Searches for important information if the user specified a file othewise 
        the standard output is iterated through
        */    
        let time: bool = self.matches.is_present("time");
        let start = Instant::now();
        let filename: &str =  self.matches.value_of("file").unwrap_or_default();
        if !filename.is_empty() {
            self.iterate_file(filename);
        } else {
            self.iterate_stdin();
        }
        if time {
            println!("Time elapsed: {:?}", start.elapsed());
        }
    }
}

fn main() -> Result<(), std::io::Error> {
    /*
    1. Creates the arguments parser
    2. Creates an instance of DataSurgeon
    3. Calls DataSurgeon.process()
    */
    let matches = App::new("DataSurgeon: https://github.com/Drew-Alleman/DataSurgeon")
        .version("1.0")
        .author("Drew Alleman")
        .about("DataSurgeon (ds) extracts sensitive information from standard output for incident response, penetration testing, and CTF challenges, including emails, credit cards, URLs, IPs, MAC addresses, and SRV DNS records. ")
        .arg(Arg::with_name("file")
            .short('f')
            .long("file")
            .help("File to extract information from")
            .takes_value(true)
        )
        .arg(Arg::with_name("junk")
            .short('j')
            .long("junk")
            .help("Attempt to remove some of the junk information that might have been sent back")
            .takes_value(false)
        )
        .arg(Arg::with_name("output")
            .short('o')
            .long("output")
            .help("Output's the results of the procedure to a local file (recommended for large files)")
            .takes_value(true)
        )
        .arg(Arg::with_name("time")
            .short('t')
            .long("time")
            .help("Time how long the operation took")
            .takes_value(false)
        )
        .arg(Arg::with_name("email")
            .short('e')
            .long("email")
            .help("Used to extract email addresses from the specifed file or output stream")
            .takes_value(false)
        )
        .arg(Arg::with_name("ip_address")
            .short('i')
            .long("ip_address")
            .help("Extracts IP addresses from the desired file")
            .takes_value(false)
        )
        .arg(Arg::with_name("ipv6_address")
            .short('6')
            .long("ipv6_address")
            .help("Extracts IPv6 addresses from the desired file")
            .takes_value(false)
        )
        .arg(Arg::with_name("mac_address")
            .short('m')
            .long("mac_address")
            .help("Extract's MAC addresses")
            .takes_value(false)
        )
        .arg(Arg::with_name("credit_card")
            .short('c')
            .long("credit_card")
            .help("Extract credit card numbers")
            .takes_value(false)
        )
        .arg(Arg::with_name("url")
            .short('u')
            .long("url")
            .help("Extract url's")
            .takes_value(false)
        )
        .arg(Arg::with_name("domain_users")
            .short('D')
            .long("domain_users")
            .help("Extract possible Windows domain user accounts")
            .takes_value(false)
        )
        .arg(Arg::with_name("files")
            .short('F')
            .long("files")
            .help("Extract filenames")
            .takes_value(false)
        )
        .arg(Arg::with_name("srv_dns")
            .short('d')
            .long("dns")
            .help("Extract Domain Name System records")
            .takes_value(false)
        )
        .get_matches();
    let mut ds = DataSurgeon {matches: matches};
    ds.process();
    Ok(())
}
