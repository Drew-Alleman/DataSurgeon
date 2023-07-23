use std::io;

// Creates a Yes/No prompt with the provided message
// 
// # Arguments
// * `&str` - Message to print with the prompt
//
// # Return
// 
// * `bool` - True if the user responded with Y or y otherwise False
fn yn_prompt(message: &str) -> bool {
    println!("[+] {}", message);

    let mut input = String::new();

    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            match input.trim().to_lowercase().as_str() {
                "y" => true,
                "n" => false,
                _ => {
                    println!("[-] Invalid input. Please enter 'y' or 'n'.");
                    yn_prompt(message) // Ask the prompt again for invalid input
                }
            }
        }
        Err(error) => {
            println!("[-] Failed to read input: {}", error);
            false
        }
    }
}


// Handles errors by printing a custom error message and prompting the user if they want to exit the program.
//
// # Arguments
//
// * `error_message` - A string slice that holds the custom error message.
//
// # Examples
//
// ```
// handle_error_and_exit("Failed to open plugins.json file.");
// ```
//
// If the user responds with 'y' to the prompt, the program will exit. If 'n' is entered, the function will return and allow the program to continue.
pub fn handle_error_and_exit(error_message: &str) {
    println!("[-] Error: {}", error_message);
    if yn_prompt("Do you want to exit? (y/n): ") {
        std::process::exit(1);
    }
}

/// Normalizes a URL string by converting it to lowercase, 
/// removing any 'http://' or 'https://' prefixes, and trimming trailing slashes.
///
/// # Arguments
///
/// * `url` - A string slice that holds the URL to be normalized.
///
/// # Example
///
/// ```
/// let url = "HTTP://www.Example.com/";
/// let normalized_url = normalize_url(url);
/// println!("{}", normalized_url); // Outputs: "www.example.com"
/// ```
pub fn normalize_url(url: &str) -> String {
    url.to_lowercase().trim_end_matches('/').replace("https://", "").replace("http://", "")
}