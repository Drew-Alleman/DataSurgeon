# DataSurgeon (WIP)
DataSurgeon (ds) is a versatile tool designed for incident response, penetration testing, and CTF challenges. It allows for the extraction of various types of sensitive information from standard output, including emails, credit cards, URLs, IP addresses, MAC addresses, and SRV DNS records.

The tool also provides support for CSV output, making it easy to integrate with other tools in your workflow. 

# Usage
```
$ ./ds -h                                    
DataSurgeon: https://github.com/Drew-Alleman/DataSurgeon 1.0
Drew Alleman
DataSurgeon (ds) extracts sensitive information from standard output for incident response,
penetration testing, and CTF challenges, including emails, credit cards, URLs, IPs, MAC addresses,
and SRV DNS records.

USAGE:
    ds [OPTIONS]

OPTIONS:
    -6, --ipv6_address    Extracts IPv6 addresses from the desired file
    -c, --credit_card     Extract credit card numbers
    -d, --dns             Extract Domain Name System records
    -e, --email           Used to extract email addresses from the specifed file or output stream
    -f, --file <file>     File to extract information from
    -h, --help            Print help information
    -i, --ip_address      Extracts IP addresses from the desired file
    -m, --mac_address     Extract's MAC addresses
    -u, --url             Extract url's
    -V, --version         Print version information
    
```
# Examples
-m is used to extract mac addresses and -e is used to extract email addresses
```
wget -qO - https://help.gnome.org/users/gnome-help/stable/net-macaddress.html.en | ./ds -e -m
mac_address:   colon. <span class="code">00:1B:44:11:3A:B7</span> is an example of a MAC address.</p>
email: <p class="action_box doc-feedback">Got a comment? Spotted an error? Found the instructions unclear? <a class="button" href="mailto:docs-feedback@gnome.org?subject=Feedback%20on%20users/gnome-help/3.38/net-macaddress.page">Send feedback about this page.</a></p>
```

# Features
* Accepts file's and input from standard output
* Fast

# Project Goals
* CSV output

# Current Extraction Features
* Emails
* Credit Cards
* URL's
* IPv4 Addresses and IPv6 addresses
* MAC Addresses
* SRV DNS Records

## Want more?
Please read the contributing guidelines then create a pull request with what information you want extracted.
