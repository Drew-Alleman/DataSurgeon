/* https://github.com/Drew-Alleman/DataSurgeon
Quickly Extracts IP's, Email Addresses, Hashes, Files, Credit Cards, Social Secuirty Numbers and more from text
*/

mod utils;
mod plugins;

use crate::plugins::{RegexPlugin, list_plugins, remove_plugins_from_url, update_all_plugins, load_plugins, add_plugin_from_url, update_plugin_from_url};

use std::io;
use clap::Arg;
use regex::Regex;
use clap::Command;
use std::vec::Vec;
use walkdir::WalkDir;
use std::time::Instant;
use std::path::{Path, Display};
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::collections::{HashSet, HashMap};


struct DataSurgeon {
    matches: clap::ArgMatches,
    plugins: Vec<RegexPlugin>,
    output_file: String,
    drop: String,
    filter: String,
    filter_regex: Regex,
    drop_regex: Regex,
    filename: String,
    directory: String,
    clean: bool,
    count: bool,
    is_output: bool,
    thorough: bool,
    hide_type: bool,
    display: bool,
    is_csv: bool,
    ignore: bool,
    line_count: i32,
}

impl Default for DataSurgeon {
    fn default() -> Self {
        let plugins = load_plugins();
        let mut app = Command::new("DataSurgeon: https://github.com/Drew-Alleman/DataSurgeon")
            .version("1.2.4")
            .author("https://github.com/Drew-Alleman/DataSurgeon")
            .about("Note: All extraction features (e.g: -i) work on a specified file (-f) or an output stream.")
        .arg(Arg::new("file")
            .short('f')
            .long("file")
            .help("File to extract information from")
            .action(clap::ArgAction::Set)
        )
        .arg(Arg::new("add")
            .long("add")
            .help("Adds a plugin from a GitHub repository (e.g ds --add https://github.com/Drew-Alleman/ds-winreg-plugin/)")
            .action(clap::ArgAction::Set)
        )
        .arg(Arg::new("update")
            .long("update")
            .help("Updates a plugin from a GitHub repository. You can also use `ds --update all` to update all plugins. (e.g ds --update https://github.com/DataSurgeon-ds/ds-cve-plugin/)")
            .action(clap::ArgAction::Set)
        )
        .arg(Arg::new("remove")
            .long("remove")
            .help("Removes a plugin from a GitHub repository (e.g ds --remove https://github.com/Drew-Alleman/ds-winreg-plugin/)")
            .action(clap::ArgAction::Set)
        )
        .arg(Arg::new("list")
            .long("list")
            .help("List all added plugins")
            .action(clap::ArgAction::SetTrue)
        )
        .arg(
            Arg::new("clean")
            .short('C')
            .long("clean")
            .help("Only displays the matched result, rather than the entire line")
            .action(clap::ArgAction::SetTrue)
        )
        .arg(
            Arg::new("directory")
            .long("directory")
            .help("Process all files found in the specified directory")
            .action(clap::ArgAction::Set)
        )
        .arg(
            Arg::new("ignore")
            .long("ignore")
            .help("Silences error messages")
            .action(clap::ArgAction::SetTrue)
        )
        .arg(
            Arg::new("count")
            .long("line")
            .short('l')
            .help("Displays the line number where the match occurred")
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
        .arg(Arg::new("drop")
             .long("drop")
             .help("Specify a regular expression to exclude certain patterns from the search. (e.g --drop \"^.{1,10}$\" will hide all matches not under 10 characters)")
             .action(clap::ArgAction::Set)
        )
        .arg(Arg::new("filter")
             .long("filter")
             .help("Include only lines that match the specified regex. (e.g: '--filter ^error' will only include lines that start with the word 'error'")
             .action(clap::ArgAction::Set)
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
        );
        let args: Vec<_> = plugins
            .iter()
            .map(|plugin| (plugin.content_type.clone(), plugin.arg_long_name.clone(), plugin.help_message.clone()))
            .collect();

        for (content_type, arg_long_name, help_message) in args {
            let content_type = Box::leak(content_type.into_boxed_str());
            let arg_long_name = Box::leak(arg_long_name.into_boxed_str());
            let help_message = Box::leak(help_message.into_boxed_str());
        
            let arg = Arg::new(&*content_type)
                .long(&*arg_long_name)
                .help(&*help_message)
                .action(clap::ArgAction::SetTrue);
            app = app.arg(arg);
        }
        let matches = app.get_matches();
        Self {
            matches,
            plugins: plugins,
            output_file: "".to_string(),
            filename: "".to_string(),
            clean: false,
            drop: "".to_string(),
            filter: "".to_string(),
            directory: "".to_string(),
            drop_regex: Regex::new(r#".{10,}"#).unwrap(),
            filter_regex: Regex::new(r#".{10,}"#).unwrap(),
            is_output: false,
            ignore: false,
            thorough: false,
            hide_type: false,
            display: false,
            is_csv: false,
            count: false,
            line_count: 0,
        }
    }
}



impl  DataSurgeon {

    fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    // Builds a regex query to search for important information
    //
    // # Return
    //
    // * `HashMap<&'static str, Regex>` - A HashMap containg the content type and the regex associated
    fn build_regex_query(&self) -> HashMap<String, Regex>{
        let mut regex_map: HashMap<String, Regex> = [
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
            ("aws_keys", Regex::new(r"\b(?:ACCESS_KEY|aws_access_key_id|aws_secret_access_key|secret_key)\s*=\s*([a-zA-Z0-9/+]{20,40})\s*\b").unwrap()),
            ("bitcoin_wallet", Regex::new(r"\b([13][a-km-zA-HJ-NP-Z1-9]{25,34})\b").unwrap()),
            // ("ssh_keys", Regex::new(r"(ssh-rsa AAAA[0-9A-Za-z+/]+[=]{0,3}( [^@]+@[^@]+)?)").unwrap())
            ("files", Regex::new(r"([\w,\s-]+\.(txt|pdf|doc|docx|xls|xlsx|xml|jpg|jpeg|png|gif|bmp|csv|json|yaml|log|tar|tgz|gz|zip|rar|7z|exe|dll|bat|ps1|sh|py|rb|js|mdb|sql|db|dbf|ini|cfg|conf|bak|old|backup|pgp|gpg|aes|dll|sys|drv|ocx|pcap|tcpdump))").unwrap()),
            ("hashes", Regex::new(r"\b([0-9a-fA-F]{32}|[0-9a-fA-F]{40}|[0-9a-fA-F]{56}|[0-9a-fA-F]{64}|[0-9a-fA-F]{96}|[0-9a-fA-F]{128}|[0-9a-fA-F]{56}|[0-9a-fA-F]{128}|[0-9a-fA-F]{224}|[0-9a-fA-F]{256}|[0-9a-fA-F]{384}|[0-9a-fA-F]{512}|[a-fA-F0-9*]{16}|[a-fA-F0-9*]{40}|[a-fA-F0-9*]{64}|[a-fA-F0-9*]{96}|[a-fA-F0-9*]{128})\b").unwrap())
        ].iter().map(|(k, v)| (k.to_string(), v.clone())).collect();
        
        for plugin in &self.plugins {
            if let Ok(regex) = Regex::new(&plugin.regex) {
                if !regex_map.contains_key(&plugin.content_type) {
                    regex_map.insert(plugin.content_type.clone(), regex);
                }
            }
        }
        let keys: Vec<String> = regex_map.keys().cloned().collect();
        let keys_to_keep: Vec<String> = keys
            .into_iter()
            .filter(|key| {
                let has_match = self.matches.get_one(&key);
                let is_empty = regex_map[key].as_str().is_empty(); // change here
                *has_match.unwrap() && !is_empty
            })
            .collect();

        let filtered_map: HashMap<String, Regex> = keys_to_keep
            .into_iter()
            .map(|key| (key.clone(), regex_map.remove(&key).unwrap()))
            .collect();

        filtered_map
    }


    // Writes content to the specified output file (-o option)
    //  
    // # Arguments
    //
    // * `&str` - Message to write to the output file
    fn write_to_file(&self, message: &str) -> () {
        let mut file = match OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.output_file) 
        {
            Ok(file) => file,
            Err(error) => match error.kind() {
                std::io::ErrorKind::NotFound => {
                    // This should not happen
                    self.print_error(format!("Failed to open output file: {}", self.filename));
                    std::process::exit(1);
                }
                std::io::ErrorKind::PermissionDenied => {
                    self.print_error(format!("Permission denied for file: {}", self.filename));
                    std::process::exit(1);
                }
                _ => {
                    self.print_error(format!("Failed to open output file: {}. Error: {}", self.filename, error));
                    std::process::exit(1);
                }
            },
        };
    
        match writeln!(file, "{}", message) {
            Ok(_) => {
                // Write successful
            },
            Err(error) => match error.kind() {
                std::io::ErrorKind::PermissionDenied => {
                    println!("Permission denied while writing to output file: {}", self.filename);
                    std::process::exit(1);
                }
                std::io::ErrorKind::WriteZero => {
                    self.print_error(format!("Disk is full, unable to write to output file: {}", self.filename));
                    std::process::exit(1);
                }
                _ => {
                    println!("Failed to write to output file: {} error: {}", self.filename, error);
                }
            }
        }
    }

    // This function is used to determine if a line should be printed
    // out or not. 
    //
    // # Return
    //
    // * `bool` - True if the line should be displayed
    // check if drop regex was set and there was a match
    fn is_worthy(&self, line: &str) -> bool {
        if !self.drop.is_empty() && self.drop_regex.is_match(line) {
            return false;
        }
        // check if filter regex was set and there was no match
        if !self.filter.is_empty() && self.filter_regex.is_match(line) {
            return true;
        }
        // no filter regex is set
        if self.filter.is_empty() {
            return true;
        }
        // a filter regex was set but there was no match
        return false;
    }

    // Searches through the specified regexes to determine if the data
    // provided is valuable information for the provided user
    //
    // # Arguments
    //
    // * `&std::io::Result<String>` - Line to process
    // * `regex_map`                - Created regexes to search through
    fn handle(&mut self, line: &std::io::Result<String>, regex_map: &HashMap<String, Regex>) {
        if let Ok(line) = line {
            self.line_count += 1;
            let mut capture_set: HashSet<String> = HashSet::new();
            for (content_type, regex) in regex_map.iter() {
                for capture in regex.captures_iter(&line) {
                    if !self.clean && self.is_worthy(&line) {
                        self.handle_message(&line, &content_type);
                        if !self.thorough {
                            return;
                        }
                    }
                    // Fetch the first member of the capture group
                    if let Some(capture_match) = capture.get(0) {
                        let filtered_capture: String = capture_match.as_str().to_string();
                        if !self.is_worthy(&filtered_capture) {
                            continue;
                        }
                        // Attempt to insert the captured item into the hashmap
                        match capture_set.insert(filtered_capture.clone()) {
                            // If we can't because the matched item was already found, move to the next match
                            false => continue,
                            true => {
                                self.handle_message(&filtered_capture.to_owned(), &content_type);
                                if !self.thorough {
                                    return;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            // Annoying
            // if let Err(error) = line {
            //     self.print_error(format!("Ran into error: {} when trying to read the line {} in {}", error, self.line_count, self.filename));
            // }
        }
    }

    // Handles the specifed line and either writes or prints it to the
    // screen
    //
    // # Arguments
    // 
    // * `&String` - The line that had intresting content on it
    // * `&str`    - The content that was matched to the line
    fn handle_message(&self, line: &String, content_type: &str) -> () {
        let message = if self.is_csv {
            match (self.hide_type, self.display, self.count) {
                (true, true, true) => format!("{}, {}, {}", self.filename, self.line_count, line),
                (true, true, false) => format!("{}, {}", self.filename, line),
                (true, false, true) => format!("{}, {}", self.line_count, line),
                (true, false, false) => format!("{}", line),
                (false, true, true) => format!("{}, {}, {}, {}", content_type, self.filename, self.line_count, line),
                (false, true, false) => format!("{}, {}, {}", content_type, self.filename, line),
                (false, false, true) => format!("{}, {}, {}", content_type, self.line_count, line),
                (false, false, false) => format!("{}, {}", content_type, line),
            }
        } else {
            match (self.hide_type, self.display, self.count) {
                (true, true, true) => format!("file: {} {}, line: {}", self.filename, line, self.line_count),
                (true, true, false) => format!("file: {} {}", self.filename, line),
                (true, false, true) => format!("{}, line: {}", line, self.line_count),
                (true, false, false) => format!("{}", line),
                (false, true, true) => format!("{}, file: {} {}, line: {}", content_type, self.filename, line, self.line_count),
                (false, true, false) => format!("{}, file: {} {}", content_type, self.filename, line),
                (false, false, true) => format!("{}, {}, line: {}", content_type, line, self.line_count),
                (false, false, false) => format!("{}: {}", content_type, line),
            }
        };
        
        if self.is_output {
            self.write_to_file(&message);
        } else {
            println!("{}", message);
        }
    }

    // Used to build the attributes in the clap args
    fn build_arguments(&mut self) -> () {
        let update_plugin_url = self.matches.get_one::<String>("update").unwrap_or(&String::new()).to_string().to_owned();
        let add_url = self.matches.get_one::<String>("add").unwrap_or(&String::new()).to_string().to_owned();
        let remove_url = self.matches.get_one::<String>("remove").unwrap_or(&String::new()).to_string().to_owned();
        let list = *self.matches.get_one::<bool>("list").unwrap_or(&false);
        if list {
            list_plugins();
            std::process::exit(1);

        }
        if !add_url.is_empty() {
            if add_plugin_from_url(&add_url) {
                println!("[*] Download and added plugin: {}", add_url.clone());
            }
            std::process::exit(1);

        }
        if !remove_url.is_empty() {
            if remove_plugins_from_url(&remove_url) {
                println!("[*] Removed plugin: {}", remove_url.clone());
            }
            std::process::exit(1);

        }
        if !update_plugin_url.is_empty() {
            if update_plugin_url.to_lowercase() == "all" {
                update_all_plugins();
            } else {
                if update_plugin_from_url(&update_plugin_url) {
                    println!("[*] Updated Plugin: {}", update_plugin_url);
                }
            }
            std::process::exit(1);
        }
        self.output_file = self.matches.get_one::<String>("output").unwrap_or(&String::new()).to_string().to_owned();
        self.is_output = !self.output_file.is_empty();
        self.clean = *self.matches.get_one::<bool>("clean").unwrap_or(&false);
        self.count = *self.matches.get_one::<bool>("count").unwrap_or(&false);
        self.thorough = *self.matches.get_one::<bool>("thorough").unwrap_or(&false);
        self.hide_type = *self.matches.get_one::<bool>("hide").unwrap_or(&false);
        self.display = *self.matches.get_one::<bool>("display").unwrap_or(&false);
        self.ignore = *self.matches.get_one::<bool>("ignore").unwrap_or(&false);
        self.filename = self.matches.get_one::<String>("file").unwrap_or(&String::new()).to_string().to_owned();
        self.directory = self.matches.get_one::<String>("directory").unwrap_or(&String::new()).to_string().to_owned();
        self.drop = self.matches.get_one::<String>("drop").unwrap_or(&String::new()).to_string().to_owned();
        self.filter = self.matches.get_one::<String>("filter").unwrap_or(&String::new()).to_string().to_owned();
        if !self.drop.is_empty() {
            self.drop_regex = Regex::new(&self.drop).unwrap();
        }
        if !self.filter.is_empty() {
            self.filter_regex = Regex::new(&self.filter).unwrap();
        }
        if self.is_output {
            let parts: Vec<&str> = self.output_file.split(".").collect();
            if parts.len() == 1 {
                self.is_csv = false;
            } else {
                let extension = parts.last().unwrap_or(&"");
                if extension.is_empty() {
                    self.is_csv = false;
                } else {
                    match extension {
                        &"csv" => {
                            self.is_csv = true;
                            self.create_headers();
                        },
                        _ => self.is_csv = false,
                    }
                }
            }
        }
    }

    // Iterates through the specified file from the -f option to find important information
    fn iterate_file(&mut self) -> () {
        match File::open(Path::new(&self.filename)) {
            Ok(file) => {
                let reader = BufReader::new(file);
                let regex_map = self.build_regex_query();
                for line in reader.lines() {
                    self.handle(&line, &regex_map);
                }
            },
            Err(error) => {
                match error.kind() {
                    std::io::ErrorKind::NotFound => {
                        self.print_error(format!("File not found: {}", self.filename));
                    },
                    std::io::ErrorKind::PermissionDenied => {
                        self.print_error(format!("Permission denied for file: {}", self.filename));
                    },
                    _ => {
                        self.print_error(format!("Error opening file: {}. Error: {}", self.filename, error));
                    }
                }
                std::process::exit(1);
            }
        }
    }

    // Prints the error to the screen unless the "--ignore" option is enabled
    // 
    // # Arguments
    //
    // * `String` -  Message to print to the screen
    fn print_error(&self, message: String) -> () {
        if !self.ignore { 
            println!("[-] {}", message);
        }
    }

    // Iterates through ALL files found in the specified directory "--directory" option 
    fn iterate_files(&mut self) -> () {
        let regex_map = self.build_regex_query();
        for entry in WalkDir::new(self.directory.clone()).into_iter() {
            match entry {
                Ok(entry) if entry.file_type().is_file() => {
                    let file = File::open(Path::new(entry.path()));
                    self.line_count = 0;
                    self.filename = entry.path().display().to_string();
                    match file {
                        Ok(file) => {
                            let reader = BufReader::new(file);
                            for line in reader.lines() {
                                self.handle(&line, &regex_map);
                            }
                        },
                        Err(error) => {
                            let filename: Display = entry.path().display();
                            match error.kind() {
                                std::io::ErrorKind::NotFound => {
                                    self.print_error(format!("File not found: {}", filename));
                                },
                                std::io::ErrorKind::PermissionDenied => {
                                    self.print_error(format!("Permission denied for file: {}", filename));
                                },
                                _ => {
                                    self.print_error(format!("Error opening file {}: {}", filename, error));
                                }
                            }
                            continue; // Continue to the next iteration of the loop
                        }
                    }
                }
                _ => continue, // Continue to the next iteration of the loop
            }
        }
    }

    
    // Creates the headers for the outputted CSV file
    fn create_headers(&self) -> () {
        let message = match (self.hide_type, self.display, self.count) {
            (true, true, true) => format!("file, line, data"),
            (true, true, false) => format!("file, data"),
            (true, false, true) => format!("line, data"),
            (true, false, false) => format!("data"),
            (false, true, true) => format!("content_type, file, line, data"),
            (false, true, false) => format!("content_type, file, data"),
            (false, false, true) => format!("content_type, line, data"),
            (false, false, false) => format!("content_type, data"),
        };
        self.write_to_file(&message);
    }

    // Iterates through the standard input to find important informatio
    fn iterate_stdin(&mut self) -> () {
        if !self.matches.get_one::<bool>("suppress").unwrap_or(&false) {
            println!("[*] Reading standard input. If you meant to analyze a file use 'ds -f <FILE>' (ctrl+c to exit)");
        }
        let stdin = io::stdin();
        let reader = stdin.lock();
        let regex_map = self.build_regex_query();
        for line in reader.lines() {
            self.handle(&line, &regex_map);
        }

    }

    // Displays how long the program took
    //
    // # Arguments
    //
    // * `f32` - Time that has elapsed
    fn display_time(&self, elapsed: f32) -> () {
        let hours: u32 = (elapsed / 3600.0) as u32;
        let minutes: u32 = ((elapsed / 60.0) as u32) % 60;
        let seconds: u32 = (elapsed as u32) % 60;
        println!("[*] Time elapsed: {:02}h:{:02}m:{:02}s", hours, minutes, seconds);
    }

    // Searches for important information if the user specified a file othewise
    // the standard output is iterated through
    fn process(&mut self) -> () {
        self.build_arguments();
        let start = Instant::now();
        if !self.filename.is_empty() {
            self.iterate_file();
        } else if !self.directory.is_empty() {
            self.iterate_files();
        } else {
            self.iterate_stdin();
        }
        if *self.matches.get_one::<bool>("time").unwrap() {
            self.display_time(start.elapsed().as_secs_f32());
        }
    }
}

// Creates the arguments parser
// Creates an instance of DataSurgeon
// Calls DataSurgeon.process()
fn main() -> Result<(), std::io::Error> {
    let mut ds = DataSurgeon::new();
    ds.process();
    Ok(())
}
