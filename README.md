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
    -6, --ipv6_address       Extracts IPv6 addresses from the desired file
    -c, --credit_card        Extract credit card numbers
    -d, --dns                Extract Domain Name System records
    -D, --domain_users       Extract possible Windows domain user accounts
    -e, --email              Used to extract email addresses from the specifed file or output stream
    -f, --file <file>        File to extract information from
    -F, --files              Extract filenames
    -h, --help               Print help information
    -i, --ip_address         Extracts IP addresses from the desired file
    -j, --junk               Attempt to remove some of the junk information that might have been
                             sent back
    -m, --mac_address        Extract's MAC addresses
    -o, --output <output>    Output's the results of the procedure to a local file (recommended for
                             large files)
    -t, --time               Time how long the operation took
    -u, --url                Extract url's
    -V, --version            Print version information
                                                      
    
```
# Examples
When ran with no arguments ds with wait for standard input. With no specified query (e.g: -url, -6) DataSurgeon search for all types of data. If your working with larger files it is often faster to use specific queries.

## Extracting Files From a Remote Webiste
--junk will attempt to remove any non critical strings from the display message
```
$ wget -qO - https://stackoverflow.com/ | ./ds --files --junk
files: apple-touch-icon.png
files: min.js
files: en.js
files: en.js
files: min.js
files: StackExchange.ini
files: en.js
files: gps.ini
files: topbar.ini
files: illo-for-you.png
files: illo-for-you.png
files: illo-home-search.png
files: illo-home-search.png
```

## Extracting Mac Addresses From an Output File
Here I am pulling all the mac addresses from the log file of [autodeauth](https://github.com/Drew-Alleman/autodeauth)
```
$ ./ds -f /var/log/autodeauth/log     
mac_address: 2023-02-26 00:28:19 - Sending 500 deauth frames to network: BC:2E:48:E5:DE:FF -- PrivateNetwork
mac_address: 2023-02-26 00:35:22 - Sending 500 deauth frames to network: 90:58:51:1C:C9:E1 -- TestNet
mac_address: 2023-02-26 00:35:40 - Sending 500 deauth frames to network: DC:EB:69:BA:79:C9 -- HomeNet
mac_address: 2023-02-26 00:35:56 - Sending 500 deauth frames to network: C4:41:1E:53:7D:8C -- TheCorp
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
