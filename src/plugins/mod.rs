
use crate::utils::{handle_error_and_exit, normalize_url};

use serde_json;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use serde::{Deserialize, Serialize};


/// Returns the path to the `plugins.json` file for the DataSurgeon application.
///
/// # Platform Specific
///
/// - On Windows, the path is hard-coded as "C:\\ds\\plugins.json".
/// - On Unix-like operating systems (including Linux and macOS), the path is dynamically determined based on the user's home directory, as stored in the `HOME` environment variable.
///
/// # Panics
///
/// - This function will panic if the `HOME` environment variable is not set on a Unix-like operating system.
/// - This function will panic if an unsupported platform is encountered.
///
/// # Returns
///
/// A `String` containing the absolute path to the `plugins.json` file.
fn get_plugin_path() -> String {
    #[cfg(target_os = "windows")]
    {
        return String::from("C:\\ds\\plugins.json");
    }

    #[cfg(unix)] // This targets all Unix-like systems, including Linux and macOS
    {
        let home_dir = std::env::var("HOME").expect("Home directory not found. Failed to read environmental variable: `HOME`.");
        return format!("{}/.DataSurgeon/plugins.json", home_dir);
    }

    #[cfg(not(any(target_os = "windows", unix)))] // Fallback for other platforms
    {
        panic!("Unsupported platform");
    }
}

const LOCAL_PLUGIN_PATH: &str = "plugins.json";
const FAILED_TO_LOCATE: &str = "Failed to open plugins.json file.";

/* This is our plugin object used to parse the items found 
in the `plugins.json` file. ALL FIELDS are required....

* `arg_long_name` must be unique.
* `content_type`is a 1 word description of what you are searching for (no spaces)

    {
        "content_type":"numbers",
        "arg_long_name": "numbers",
        "help_message": "Extracts numbers",
        "regex": "(\\d+)",
        "source_url": "https://github.com/Drew-Alleman/ds-test-plugin/"
    }

*/
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct RegexPlugin {
    pub arg_long_name: String,
    pub help_message: String,
    pub content_type: String,
    pub source_url: String,
    pub regex: String,
}


// Fetches and parses plugins from a given URL.
//
// This function does the following:
// 1. Sends a GET request to the given URL and gets the response.
// 2. Extracts the body of the response as text.
// 3. Parses the body text into a vector of `RegexPlugin` objects.
//
// # Arguments
//
// * `url` - A string slice that holds the URL of the plugins.
//
/// # Returns
//
// * `Result<Vec<RegexPlugin>, Box<dyn std::error::Error>>` - Returns a `Result` which is an `Ok` of a vector of `RegexPlugin` objects if the plugins are fetched and parsed successfully, or an `Err` of a dynamic error if there is any error in fetching or parsing the plugins.
pub fn get_plugins_from_url(url: &str) -> Result<Vec<RegexPlugin>, Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 6.3; WOW64; Trident/7.0; Touch; MAGWJS; rv:11.0) like Gecko")
        .build()?;

    let resp = client.get(url).send()?;
    
    if !resp.status().is_success() {
        println!("Error: Received a {} status code from the server.", resp.status());
        std::process::exit(1);
    }

    let body = resp.text()?;
    let plugins: Vec<RegexPlugin> = serde_json::from_str(&body)?;
    Ok(plugins)
}


// This function tries to find a plugins.json file at certain paths.
//
// On a Windows system, it checks at the path C:\ds\plugins.json.
// On a Linux system, it checks at the path /usr/local/bin/ds/plugins.json.
// If the file is not found at the system-specific path, it checks the local path ./plugins.json.
//
// If a plugins.json file is found at any of these paths, the function returns the path as a String wrapped in Some.
// If no plugins.json file is found at any of these paths, the function returns None.
pub fn find_plugin_file() -> Option<String> {
    let path: String = get_plugin_path();
    if Path::new(&path).exists() {
        return Some(path);
    } else if Path::new(LOCAL_PLUGIN_PATH).exists() {
        return Some(String::from(LOCAL_PLUGIN_PATH));
    }
    None
}


// Loads plugins from a JSON file.
//
// This function attempts to find and open a `plugins.json` file. If the file is found and opened successfully,
// it reads the file content into a string and attempts to parse that string into a vector of `RegexPlugin` objects.
//
// If the file cannot be found, opened, or parsed successfully, it prompts the user to decide whether to exit the program or continue.
// If the user chooses to continue, it returns an empty vector.
//
// # Returns
//
// * `Vec<RegexPlugin>` - A vector of `RegexPlugin` objects. If the `plugins.json` file cannot be found, opened, or parsed successfully,
// and the user chooses to continue, this will be an empty vector.
//
// # Panics
//
// This function will panic if it fails to open the `plugins.json` file or read its content into a string,
// or if the user chooses to exit when prompted after a failure to find, open, or parse the `plugins.json` file.
pub fn load_plugins() -> Vec<RegexPlugin> {
    match find_plugin_file() {
        Some(filename) => {
            let mut file = match File::open(&filename) {
                Ok(file) => file,
                Err(_) => {
                    handle_error_and_exit(FAILED_TO_LOCATE);
                    return Vec::new();
                }
            };
            let mut data = String::new();
            if let Err(_) = file.read_to_string(&mut data) {
                handle_error_and_exit("Failed to read from plugins.json file.");
                return Vec::new();
            }
            match serde_json::from_str(&data) {
                Ok(plugins) => plugins,
                Err(_) => {
                    handle_error_and_exit("Failed to parse plugins.json. Please ensure all required fields are present.");
                    Vec::new() // Return an empty list of plugins if the user doesn't want to exit.
                }
            }
        },
        None => {
            handle_error_and_exit(&format!("{} PLUGIN_PATH: {}", FAILED_TO_LOCATE, get_plugin_path()));
            Vec::new() // Return an empty list of plugins if the user doesn't want to exit.
        }
    }
}


// Adds a new plugin from a given URL to the existing plugins.
//
// This function does the following:
// 1. Formats the URL to point to the raw plugins.json file.
// 2. Fetches the plugins from the URL and parses them.
// 3. Loads the existing plugins.
// 4. Checks if any of the new plugins already exist in the existing plugins.
// 5. If a new plugin doesn't exist in the existing plugins, it is added.
// 6. Saves the merged plugins back to the plugins.json file.
//
// # Arguments
//
// * `url` - A string slice that holds the URL of the new plugin.
//
// # Returns
//
// * `bool` - Returns true if the new plugin was added successfully, false otherwise.
//
// # Panics
//
// The function will panic if it fails to parse plugins.json from the provided URL,
// if it fails to find the plugins.json file, if it fails to open the plugins.json file for writing,
// or if it fails to write to the plugins.json file.
pub fn add_plugin_from_url(url: &str) -> bool {
    // Format the URL to point to the raw plugins.json file
    let raw_url = format!("{}/main/plugins.json", url.trim_end_matches('/'));
    let url = raw_url.replace("github.com", "raw.githubusercontent.com");

    let plugins: Vec<RegexPlugin> = match get_plugins_from_url(&url) {
        Ok(mut plugins) => {
            for plugin in &mut plugins {
                // Update the source_url of the plugin to match the provided URL
                plugin.source_url = url.trim_end_matches("/main/plugins.json").to_string();
            }
            plugins
        },
        Err(e) => {
            eprintln!("[-] Error: Failed to parse plugins.json from the provided URL. Reason: {}", e);
            std::process::exit(1);
        }
    };

    // Load the existing plugins
    let mut existing_plugins = load_plugins();

    // Check if any of the new plugins already exist
    for new_plugin in &plugins {
        for existing_plugin in &existing_plugins {
            if new_plugin.arg_long_name == existing_plugin.arg_long_name {
                println!("[-] Error: Skipping Duplicate plugin found with arg_long_name: {}", new_plugin.arg_long_name);
                if plugins.len() == 1 {
                    return false;
                }
                continue
            }
        }
    }
    // Merge the new plugins with the existing plugins
    existing_plugins.extend(plugins);

    // Save the merged plugins back to the plugins.json file
    let plugin_file_path = match find_plugin_file() {
        Some(path) => path,
        None => {
            println!("[-] Error: {}", FAILED_TO_LOCATE);
            std::process::exit(1);
        }
    };

    let file = match File::create(plugin_file_path) {
        Ok(file) => file,
        Err(_) => {
            println!("[-] Error: Failed to open plugins.json file for writing.");
            std::process::exit(1);
        }
    };

    if let Err(_) = serde_json::to_writer(file, &existing_plugins) {
        println!("[-] Error: Failed to write to plugins.json file.");
        std::process::exit(1);
    }
    true
}


/// Removes the plugins associated with a given URL.
///
/// This function loads the existing plugins, checks if any of them were added
/// from the specified URL, and if so, removes them. It then saves the updated
/// plugins back to the plugins.json file. If any operation fails during this
/// process, an error message is printed and the program is terminated.
///
/// # Arguments
///
/// * `url` - The URL associated with the plugins to be removed.
///
/// # Returns
///
/// * `bool` - Returns `true` if any plugins were removed, and `false` otherwise.
pub fn remove_plugins_from_url(url: &str) -> bool {
    // Load the existing plugins
    let url = url.trim_end_matches('/');
    let mut existing_plugins = load_plugins();

    // Check if any of the existing plugins were added from the specified URL
    let mut removed = false;
    for i in (0..existing_plugins.len()).rev() {
        let normalized_url = normalize_url(&url);
        let plugin_url = normalize_url(&existing_plugins[i].source_url);
        if plugin_url == normalized_url {
            // Remove the plugin
            existing_plugins.remove(i);
            removed = true;
        }
    }

    if removed {
        // Save the updated plugins back to the plugins.json file
        let plugin_file_path = match find_plugin_file() {
            Some(path) => path,
            None => {
                println!("[-] Error: {}", FAILED_TO_LOCATE);
                std::process::exit(1);
            }
        };

        let file = match File::create(plugin_file_path) {
            Ok(file) => file,
            Err(_) => {
                println!("[-] Error: Failed to open plugins.json file for writing.");
                std::process::exit(1);
            }
        };

        if let Err(_) = serde_json::to_writer(file, &existing_plugins) {
            println!("[-] Error: Failed to write to plugins.json file.");
            std::process::exit(1);
        }
    }

    removed
}

// This function loads all plugins thens lists them to the user in a table
pub fn list_plugins() {
    let existing_plugins = load_plugins();
    let plugin_path: String = get_plugin_path();
    // Check if there are no plugins
    if existing_plugins.is_empty() {
        println!("No plugins found. Plugin File: {}", &plugin_path);
        return;
    }
    println!("Plugin File: {}\n", plugin_path);

    // Define the widths of the columns
    let url_width = existing_plugins
        .iter()
        .max_by_key(|p| p.source_url.len())
        .unwrap()
        .source_url
        .len();
    let arg_width = existing_plugins
        .iter()
        .max_by_key(|p| p.arg_long_name.len())
        .unwrap()
        .arg_long_name
        .len();

    println!("{:<width$} | {:<arg_width$}", "Source URL", "Argument Long Name", width = url_width, arg_width = arg_width);
    for plugin in &existing_plugins {
        println!("{:<width$} | {:<arg_width$}", plugin.source_url, plugin.arg_long_name, width = url_width, arg_width = arg_width);
    }
}
