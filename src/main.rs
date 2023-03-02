use std::io;
use std::vec::Vec;
use std::io::{BufRead, BufReader, Write};
use std::fs::{File, OpenOptions};
use std::path::Path;
use std::collections::HashMap;
use std::time::Instant;
use regex::Regex;
use clap::{Arg};
use clap::Command;

struct DataSurgeon {
    matches: clap::ArgMatches,
    output_file: String,
    filename: String,
    clean: bool,
    is_output: bool,
    thorough: bool,
    hide_type: bool,
}


impl Default for DataSurgeon {
    fn default() -> Self {
        Self {
            matches: Command::new("DataSurgeon: https://github.com/Drew-Alleman/DataSurgeon")
        .version("1.0")
        .author("Drew Alleman")
        .about("DataSurgeon (ds) extracts sensitive information from standard input for incident response, penetration testing, and CTF challenges, including emails, credit cards, URLs, IPs, MAC addresses, and SRV DNS records. ")
        .arg(Arg::new("file")
            .short('f')
            .long("file")
            .help("File to extract information from")
            .action(clap::ArgAction::Set)
        )
        .arg(Arg::new("clean")
            .short('C')
            .long("clean")
            .help("Attempt to remove some of the clean information that might have been sent back")
            .action(clap::ArgAction::SetTrue)
        )
        .arg(Arg::new("thorough")
            .short('T')
            .long("thorough")
            .help("Continues searching for all selected matches in each row, even if multiple types of matches are found. By default, the program stops at the first match found in each row. (Slower) (Good for CSV's and JSON files)")
            .action(clap::ArgAction::SetTrue)
        )
        .arg(Arg::new("hide")
            .short('X')
            .long("hide")
            .help("Hides the identifier string infront of the desired content (e.g: 'hash: ', 'url: ', 'email: ' will not be displayed.")
           .action(clap::ArgAction::SetTrue)         
        )
        .arg(Arg::new("output")
            .short('o')
            .long("output")
            .help("Output's the results of the procedure to a local file (recommended for large files)")
            .action(clap::ArgAction::Set)
        )
        .arg(Arg::new("time")
            .short('t')
            .long("time")
            .help("Time how long the operation took")
            .action(clap::ArgAction::SetTrue) 
        )
        .arg(Arg::new("email")
            .short('e')
            .long("email")
            .help("Used to extract email addresses from the specifed file or output stream")
            .action(clap::ArgAction::SetTrue)
        )
        .arg(Arg::new("hashes")
            .short('H')
            .long("hash")
            .help("Used to extract supported hashes (NTLM, LM, bcrypt, Oracle, MD5, SHA-1, SHA-224, SHA-256, SHA-384, SHA-512, SHA3-224, SHA3-256, SHA3-384, SHA3-512, MD4) from the specified file or output stream")
            .action(clap::ArgAction::SetTrue)       
        )
        .arg(Arg::new("ip_address")
            .short('i')
            .long("ip-addr")
            .help("Extracts IP addresses from the desired file")
            .action(clap::ArgAction::SetTrue)
        )
        .arg(Arg::new("ipv6_address")
            .short('6')
            .long("ipv6-addr")
            .help("Extracts IPv6 addresses from the desired file")
            .action(clap::ArgAction::SetTrue)
        )
        .arg(Arg::new("mac_address")
            .short('m')
            .long("mac-addr")
            .help("Extract's MAC addresses")
            .action(clap::ArgAction::SetTrue)
        )
        .arg(Arg::new("credit_card")
            .short('c')
            .long("credit-card")
            .help("Extract credit card numbers")
            .action(clap::ArgAction::SetTrue)
        )
        .arg(Arg::new("url")
            .short('u')
            .long("url")
            .help("Extract url's")
            .action(clap::ArgAction::SetTrue)
        )
        .arg(Arg::new("files")
            .short('F')
            .long("files")
            .help("Extract filenames")
            .action(clap::ArgAction::SetTrue)
        )
        .arg(Arg::new("srv_dns")
            .short('d')
            .long("dns")
            .help("Extract Domain Name System records")
            .action(clap::ArgAction::SetTrue)
        )
        .get_matches(),
            output_file: "".to_string(),
            filename: "".to_string(),
            clean: false,
            is_output: false,
            thorough: false,
            hide_type: false,
        }
    }
}


impl  DataSurgeon {

    fn new() -> Self {
        Self { 
            ..Default::default() 
        }
    }

    fn build_regex_query(&self) -> HashMap<&'static str, Regex>{
        /* Builds a regex query to search for important information 
        :return: A HashMap containg the content type and the regex associated 

        Hello, Contributers! 
        To add a new regex, add a new raw_line to the following line. 
        The key is the name of the content you are searching for, 
        and the value is the associated regex.

        ALL REGEXES MUST HAVE THE TARGET ITEM IN THE FIRST CAPTURE GROUP (just use chatGPT)

        let regex_map: HashMap<&str, Regex> = [
                ("test_regex", Regex::new(r"\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Z|a-z]{2,}\b").unwrap()),
            ].iter().cloned().collect();

        The key is also used to display to the user what was found, so make it clear and concise, e.g., "email_address: Matched content."
        Note that the regex patterns must conform to Rust's regex syntax. You can test your regex patterns at https://regexr.com/.
        */
        let regex_map: HashMap<&str, Regex> = [
            ("credit_card", Regex::new(r"\b(\d{4}[- ]?\d{4}[- ]?\d{4}[- ]?\d{4})\b").unwrap()),
            ("email", Regex::new(r"\b([A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,4})\b").unwrap()),
            ("url", Regex::new(r"((?:https?|ftp)://[^\s/$.?#].[^\s]*)").unwrap()),
            ("ip_address", Regex::new(r"\b((?:\d{1,3}\.){3}\d{1,3})\b").unwrap()),
            ("ipv6_address", Regex::new(r"([0-9a-fA-F]{1,4}(:[0-9a-fA-F]{1,4}){7})").unwrap()),
            ("srv_dns", Regex::new(r"\b(.+?)\s+IN\s+SRV\s+\d+\s+\d+\s+\d+\s+(.+)\b").unwrap()),
            ("mac_address", Regex::new(r"([0-9a-fA-F]{2}(:[0-9a-fA-F]{2}){5})").unwrap()),
            ("files", Regex::new(r"([\w,\s-]+\.(txt|pdf|doc|docx|xls|xlsx|xml|jpg|jpeg|png|gif|bmp|csv|json|yaml|log|tar|tgz|gz|zip|rar|7z|exe|dll|bat|ps1|sh|py|rb|js|mdb|sql|db|dbf|ini|cfg|conf|bak|old|backup|pgp|gpg|aes|dll|sys|drv|ocx|pcap|tcpdump))").unwrap()),
            ("hashes", Regex::new(r"\b([0-9a-fA-F]{32}|[0-9a-fA-F]{40}|[0-9a-fA-F]{56}|[0-9a-fA-F]{64}|[0-9a-fA-F]{96}|[0-9a-fA-F]{128}|[0-9a-fA-F]{56}|[0-9a-fA-F]{128}|[0-9a-fA-F]{224}|[0-9a-fA-F]{256}|[0-9a-fA-F]{384}|[0-9a-fA-F]{512}|[a-fA-F0-9*]{16}|[a-fA-F0-9*]{40}|[a-fA-F0-9*]{64}|[a-fA-F0-9*]{96}|[a-fA-F0-9*]{128})\b").unwrap())
        ].iter().cloned().collect();
        let keys: Vec<&str> = regex_map.keys().copied().collect();
        /*
        If the user didn't specify any extraction choices (e.g: email, url, ip_address)
        */
        if keys.iter().all(|value_name| !self.matches.get_one::<bool>(value_name).unwrap()) {
            return regex_map;
        }
        /*
        If they did, then remove the ones they didnt select
        */
        let filtered_map: HashMap<&str, Regex> = keys
            .into_iter()
            .filter(|&key| {
                let has_match = self.matches.get_one(key); 
                let is_empty = regex_map[key].as_str().is_empty();
                *has_match.unwrap() && !is_empty

            })
            .map(|key| (key, regex_map[key].clone()))
            .collect();
        filtered_map
    }

    fn write_to_file(&self, message: String) {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.output_file)
            .expect("Failed to open output file");

        writeln!(file, "{}", message).expect("Failed to write to output file");
    }

    // fn to_row(&self) -> String {
    //     /*
    //     Converts the line to a CSV row
    //     */
    //     format!("{},{}", self.content_type, self.raw_line)
    // }

    fn handle(&self, line: &std::io::Result<String>, regex_map: &HashMap<&'static str, Regex>) -> () {
        /* Searches through the specified regexes to determine if the data 
        provided is valuable information for the provided user
        :param line: Line to process
        :param regex_map: Created regexes to search through
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
        for (content_type, regex) in regex_map.iter() {
            let mut capture_list: Vec<regex::Match> = Vec::new();
            if let Some(captures) = regex.captures(line) {
                for capture in captures.iter() {
                    if let Some(capture) = capture {
                        // Select capture group and strip all whitespaces if --clean
                        if capture_list.contains(&capture) {
                            continue;
                        }
                        if self.clean {
                            self.handle_message(&capture.as_str().to_string(), &content_type);
                        } else {
                            self.handle_message(&line, &content_type);
                        }
                        capture_list.push(capture);
                    }
                }
            }
            if !self.thorough {
                break;
        }
    }
}


    fn handle_message(&self, line: &String, content_type: &str) {
        /* Prints or Writes a message to the user
        :param message: Message to display or print
        */
        let message: String;
        let text: &str;
        let filtered_line: String;
        if self.clean {
            filtered_line = line.as_str().chars().filter(|c| !c.is_whitespace()).collect::<String>();
            text = &filtered_line;
        } else { 
            text = line;
        }
        if self.hide_type {
            message = format!("{}", text);
        } else {
            message = format!("{}: {}", content_type, text);
        }
        if self.is_output {
            self.write_to_file(message +"\n");
            return;
            }
            print!("{}\n", message); 
        }

    fn build_arguments(&mut self) {
        /*
        Used to build the attributes in the clap args
        */
        self.output_file = self.matches.get_one::<String>("output").unwrap_or(&String::new()).to_string().to_owned();
        self.is_output =  !self.output_file.is_empty();
        self.clean = *self.matches.get_one::<bool>("clean").clone().unwrap();
        self.thorough =  *self.matches.get_one::<bool>("thorough").clone().unwrap();
        self.hide_type = *self.matches.get_one::<bool>("hide").clone().unwrap();
        self.filename = self.matches.get_one::<String>("file").unwrap_or(&String::new()).to_string().to_owned();
    }


    fn iterate_file(&mut self) {
        /* Iterates through the specified file to find important information
        :param path: file to process
        */
        let file = File::open(Path::new(self.filename.as_str())).unwrap();
        let reader = BufReader::new(file);
        let regex_map = self.build_regex_query();
        for line in reader.lines() {
            self.handle(&line, &regex_map);
        }

    }

    fn iterate_stdin(&mut self) {
        /* Iterates through the standard input to find important informatio
        :param path: file to process
        */
        println!("[*] Reading standard input. If you meant to analyze a file use 'ds -f <FILE>' (ctrl+c to exit)");
        let stdin = io::stdin();
        let reader = stdin.lock();
        let regex_map = self.build_regex_query();
        for line in reader.lines() {
            self.handle(&line, &regex_map);
        }

    }

    fn display_time(&self, elapsed: f32) -> () {
        /* Displays how long the program took
        :param elapsed: Time in f32 that has elapsed.
        */    
        let hours = (elapsed / 3600.0) as u32;
        let minutes = ((elapsed / 60.0) as u32) % 60;
        let seconds = (elapsed as u32) % 60;
        let hours12 = if hours == 0 { 0 } else if hours > 12 { hours - 12 } else { hours };
        println!("Time elapsed: {:02}h:{:02}m:{:02}s", hours12, minutes, seconds);
    }

    fn process(&mut self) {
        /* Searches for important information if the user specified a file othewise 
        the standard output is iterated through
        */    
        self.build_arguments();
        let start = Instant::now();
        if !self.filename.is_empty() {
            self.iterate_file();
        } else {
            self.iterate_stdin();
        }
        if *self.matches.get_one::<bool>("time").unwrap() {
            self.display_time(start.elapsed().as_secs_f32());
        }
    }
}

fn main() -> Result<(), std::io::Error> {
    /*
    1. Creates the arguments parser
    2. Creates an instance of DataSurgeon
    3. Calls DataSurgeon.process()
    */
    let mut ds = DataSurgeon::new();
    ds.process();
    Ok(())
}
