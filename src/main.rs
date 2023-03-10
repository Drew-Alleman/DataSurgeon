/* https://github.com/Drew-Alleman/DataSurgeon
Quickly Extracts IP's, Email Addresses, Hashes, Files, Credit Cards, Social Secuirty Numbers and more from text 
*/ 
use std::io;
use clap::Arg;
use regex::Regex;
use clap::Command;
use std::vec::Vec;
use std::path::Path;
use std::time::Instant;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::collections::{HashSet, HashMap};


struct DataSurgeon {
    matches: clap::ArgMatches,
    output_file: String,
    filename: String,
    clean: bool,
    is_output: bool,
    thorough: bool,
    hide_type: bool,
    display: bool,
    is_csv: bool,
}


impl Default for DataSurgeon {
    fn default() -> Self {
        Self {
            matches: Command::new("DataSurgeon: https://github.com/Drew-Alleman/DataSurgeon")
        .version("1.0.6")
        .author("https://github.com/Drew-Alleman/DataSurgeon")
        .about("Note: All extraction features (e.g: -i) work on a specified file (-f) or an output stream.")
        .arg(Arg::new("file")
            .short('f')
            .long("file")
            .help("File to extract information from")
            .action(clap::ArgAction::Set)
        )
        .arg(
            Arg::new("clean")
            .short('C')
            .long("clean")
            .help("Only displays the matched result, rather than the entire line")
            .action(clap::ArgAction::SetTrue)
        )
        .arg(Arg::new("thorough")
            .short('T')
            .long("thorough")
            .help("Doesn't stop at first match (useful for -C if multiple unique matches are on the same line")
            .action(clap::ArgAction::SetTrue)
        )
        .arg(Arg::new("display")
            .short('D')
            .long("display")
            .help("Displays the filename assoicated with the content found (https://github.com/Drew-Alleman/DataSurgeon#reading-all-files-in-a-directory)")
            .action(clap::ArgAction::SetTrue)
        )
        .arg(Arg::new("suppress")
            .short('S')
            .long("suppress")
            .help("Suppress the 'Reading standard input' message when not providing a file")
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
            .help("Extract email addresses")
            .action(clap::ArgAction::SetTrue)
        )
        .arg(Arg::new("phone_number")
            .short('p')
            .long("phone")
            .help("Extracts phone numbers")
            .action(clap::ArgAction::SetTrue)
        )
        .arg(Arg::new("hashes")
            .short('H')
            .long("hash")
            .help("Extract hashes (NTLM, LM, bcrypt, Oracle, MD5, SHA-1, SHA-224, SHA-256, SHA-384, SHA-512, SHA3-224, SHA3-256, SHA3-384, SHA3-512, MD4)")
            .action(clap::ArgAction::SetTrue)       
        )
        .arg(Arg::new("ip_address")
            .short('i')
            .long("ip-addr")
            .help("Extract IP addresses")
            .action(clap::ArgAction::SetTrue)
        )
        .arg(Arg::new("ipv6_address")
            .short('6')
            .long("ipv6-addr")
            .help("Extract IPv6 addresses")
            .action(clap::ArgAction::SetTrue)
        )
        .arg(Arg::new("mac_address")
            .short('m')
            .long("mac-addr")
            .help("Extract MAC addresses")
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
            .help("Extract urls")
            .action(clap::ArgAction::SetTrue)
        )
        .arg(Arg::new("files")
            .short('F')
            .long("files")
            .help("Extract filenames")
            .action(clap::ArgAction::SetTrue)
        )
        .arg(Arg::new("bitcoin_wallet")
            .short('b')
            .long("bitcoin")
            .help("Extract bitcoin wallets")
            .action(clap::ArgAction::SetTrue)
        )
        .arg(Arg::new("aws_keys")
            .short('a')
            .long("aws")
            .help("Extract AWS keys")
            .action(clap::ArgAction::SetTrue)
        )
        .arg(Arg::new("google")
            .short('g')
            .long("google")
            .help("Extract Google service account private key ids (used for google automations services)")
            .action(clap::ArgAction::SetTrue)
        )
        // .arg(Arg::new("ssh_keys")
        //     .short('S')
        //     .long("ssh")
        //     .help("Extract ssh keys")
        //     .action(clap::ArgAction::SetTrue)
        // )
        .arg(Arg::new("srv_dns")
            .short('d')
            .long("dns")
            .help("Extract Domain Name System records")
            .action(clap::ArgAction::SetTrue)
        )
        .arg(Arg::new("social_security")
            .short('s')
            .long("social")
            .help("Extract social security numbers")
            .action(clap::ArgAction::SetTrue)
        )
        .get_matches(),
            output_file: "".to_string(),
            filename: "".to_string(),
            clean: false,
            is_output: false,
            thorough: false,
            hide_type: false,
            display: false,
            is_csv: false,
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
                ("test_regex", Regex::new(r"\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Z|a-z]{2,}\b").unwrap()), <--- Make sure to add the .unwrap() at the end of the regex
            ].iter().cloned().collect();

        The key is also used to display to the user what was found, so make it clear and concise, e.g., "email_address: Matched content."
        Note that the regex patterns must conform to Rust's regex syntax. You can test your regex patterns at https://regexr.com/.
        */
        let regex_map: HashMap<&str, Regex> = [
            ("credit_card", Regex::new(r"\b(\d{4}[- ]?\d{4}[- ]?\d{4}[- ]?\d{4})\b").unwrap()),
            ("email", Regex::new(r"\b([A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,4})\b").unwrap()),
            ("url", Regex::new(r#"(https?://(?:[^\s.,;:"'<>()\[\]{}]+\.)*[^\s.,;:"'<>()\[\]{}]+(/[^\s]*[^\s.,;:"'<>()\[\]{}\s])?)"#).unwrap()),
            ("ip_address", Regex::new(r"\b((?:\d{1,3}\.){3}\d{1,3})\b").unwrap()),
            ("social_security", Regex::new(r"\b(\d{3}-\d{2}-\d{4})\b").unwrap()),
            ("ipv6_address", Regex::new(r"([0-9a-fA-F]{1,4}(:[0-9a-fA-F]{1,4}){7})").unwrap()),
            ("phone_number", Regex::new(r"(\b[2-9]\d{2}-\d{3}-\d{4}\b)").unwrap()),
            ("srv_dns", Regex::new(r"\b(.+?)\s+IN\s+SRV\s+\d+\s+\d+\s+\d+\s+(.+)\b").unwrap()),
            ("mac_address", Regex::new(r"([0-9a-fA-F]{2}(:[0-9a-fA-F]{2}){5})").unwrap()),
            ("google", Regex::new(r#""private_key_id":\s*"(\w{40})""#).unwrap()),
            ("aws_keys", Regex::new(r"^(?i:ACCESS_KEY|aws_access_key_id|access_key|aws_secret_access_key|secret_key|aws_session_token)=(\S{20,})$").unwrap()),
            ("bitcoin_wallet", Regex::new(r"\b([13][a-km-zA-HJ-NP-Z1-9]{25,34})\b").unwrap()),
            // ("ssh_keys", Regex::new(r"(ssh-rsa AAAA[0-9A-Za-z+/]+[=]{0,3}( [^@]+@[^@]+)?)").unwrap())
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
        /* Writes content to the specified output file (-o)
        :param message: Message to write
        */
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.output_file)
            .expect("Failed to open output file");

        writeln!(file, "{}", message).expect("Failed to write to output file");
    }

    fn handle(&self, line: &std::io::Result<String>, regex_map: &HashMap<&'static str, Regex>) -> () {
        /* Searches through the specified regexes to determine if the data
        provided is valuable information for the provided user
        :param line: Line to process
        :param regex_map: Created regexes to search through
        */
        if let Ok(line) = line {
            let mut capture_set: HashSet<String> = HashSet::new();
            for (content_type, regex) in regex_map.iter() {
                for capture in regex.captures_iter(&line) {
                    if !self.clean {
                        self.handle_message(&line, &content_type);
                        if !self.thorough {
                            return;
                        }
                    }
                    if let Some(capture_match) = capture.get(0) {
                        let filtered_capture = capture_match.as_str().clone().to_string();
                        // Attempt to insert the captured item into the hashmap
                        match capture_set.insert(filtered_capture.clone()) {
                            // If we can't because the matched item was already found, move to the next
                            false => continue,
                            true => {
                                self.handle_message(&filtered_capture.clone(), &content_type);
                                if !self.thorough {
                                    return;
                                }
                            }
                        }
                    }
                }
            }
        }
    }


    fn handle_message(&self, line: &String, content_type: &str) {
        /* Prints or Writes a message to the user
        :param message: Message to display or print
        */
        let message: String;
        if self.is_csv {
            message = match (self.hide_type, self.display) {
                (true, true) => format!("{}, {}", self.filename, line),
                (true, false) => format!("{}", line),
                (false, true) => format!("{}, {}, {}", content_type, self.filename, line),
                (false, false) => format!("{}, {}", content_type, line),
            };
        } else {
            message = match (self.hide_type, self.display) {
            (true, true) => format!("{}: {}", self.filename, line),
            (true, false) => format!("{}", line),
            (false, true) => format!("{}, {}: {}",content_type, self.filename, line),
            (false, false) => format!("{}: {}", content_type, line),
            };
        }

        if self.is_output {
            self.write_to_file(message);
        } else {
            writeln!(std::io::stdout(), "{}", message).unwrap();
        }
    }

    fn build_arguments(&mut self) {
        /*
        Used to build the attributes in the clap args
        */
        self.output_file = self.matches.get_one::<String>("output").unwrap_or(&String::new()).to_string().to_owned();
        self.is_output = !self.output_file.is_empty();
        self.clean = *self.matches.get_one::<bool>("clean").unwrap_or(&false);
        self.thorough = *self.matches.get_one::<bool>("thorough").unwrap_or(&false);
        self.hide_type = *self.matches.get_one::<bool>("hide").unwrap_or(&false);
        self.display = *self.matches.get_one::<bool>("display").unwrap_or(&false);
        self.filename = self.matches.get_one::<String>("file").unwrap_or(&String::new()).to_string().to_owned();
        if self.is_output {
            let parts = self.output_file.split(".");
            let extension = parts.last().unwrap_or("");
            match extension {
                "csv" => {
                    self.is_csv = true;
                    self.create_headers();
                },
                _ => self.is_csv = false,
            };
        }
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

    fn create_headers(&self) {
        let message = match (self.hide_type, self.display) {
            (true, true) => format!("file, data"),
            (true, false) => format!("data"),
            (false, true) => format!("content_type, file, data"),
            (false, false) => format!("content_type, data"),
        };
        self.write_to_file(message)
    }

    fn iterate_stdin(&mut self) {
        /* Iterates through the standard input to find important informatio
        :param path: file to process
        */
        if !self.matches.get_one::<bool>("suppress").unwrap() {
            println!("[*] Reading standard input. If you meant to analyze a file use 'ds -f <FILE>' (ctrl+c to exit)");
        }
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
