# DataSurgeon (WIP)
DataSurgeon (ds) is a versatile tool designed for incident response, penetration testing, and CTF challenges. It allows for the extraction of various types of sensitive information from standard output, including emails, credit cards, URLs, IP addresses, MAC addresses, and SRV DNS records.

The tool also provides support for CSV output, making it easy to integrate with other tools in your workflow. 

# Usage
```
DataSurgeon: https://github.com/Drew-Alleman/DataSurgeon 1.0
Drew Alleman
DataSurgeon (ds) extracts sensitive information from standard output for incident response,
penetration testing, and CTF challenges, including emails, credit cards, URLs, IPs, MAC addresses,
and SRV DNS records.

USAGE:
    ds [OPTIONS]

OPTIONS:
    -c, --credit_card        Extract credit card numbers from the target stream/file
    -e, --email              Extract emails from the target stream/file
    -f, --file <file>        File to extract information from
    -h, --help               Print help information
    -i, --ip_address         Extract IP addresses from the target stream/file
    -m, --mac_address        Extract MAC addresses from the target stream/file
    -o, --output <output>    Output's the results of the procedure to a output file (recommended for
                             large files)
    -s, --srv                Extract SRV DNS records from the target stream/file
    -u, --url                Extract url's from the target stream/file
    -V, --version            Print version information

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
* IP Addresses
* MAC Addresses
* SRV DNS Records

## Want more?
Please read the contributing guidelines then create a pull request with what information you want extracted.
