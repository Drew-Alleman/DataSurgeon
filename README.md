# DataSurgeon (WIP)
DataSurgeon (ds) is a versatile tool designed for incident response, penetration testing, and CTF challenges. It allows for the extraction of various types of sensitive information including emails, hashes, credit cards, windows domain users, URLs, IP addresses, MAC addresses, and SRV DNS records.

# Quick Links
* [Features](#features)
* [Quick Install](#quick-install)
* [Command Line Arguments](#command-line-arguments)
* [Examples](#examples)
* [Speed Tests](#speed-tests)
* [Project Goals](#project-goals)

# Features
* Supports Windows, Linux and MacOS
* Fast Proccessing Speeds
* Accepts files
* Can process standard input
* Extracted information can be output to a secondary file


## Extractable Information 
* Emails
* Files
* Credit Cards
* URL's
* Windows Domain Usernames
* IPv4 Addresses and IPv6 addresses
* MAC Addresses
* SRV DNS Records
* Extract Hashes
    - MD4 & MD5
    - SHA-1, SHA-224, SHA-256, SHA-384, SHA-512
    - SHA-3 224, SHA-3 256, SHA-3 384, SHA-3 512
    - MySQL 323, MySQL 41
    - NTLM
    - Kerberos 5
    - PostgreSQL

### Want more? 
Please read the contributing guidelines [here](https://github.com/Drew-Alleman/DataSurgeon/blob/main/CONTRIBUTING.md#adding-a-new-regex--extraction-feature)

# Quick Install
Installds 
[Rust](https://www.rust-lang.org/tools/install) and [Github](https://desktop.github.com/)
### Linux
```
wget -O - https://raw.githubusercontent.com/Drew-Alleman/DataSurgeon/main/install/install.sh | bash
```

### Windows 
Enter the line below in an elevated powershell window. 
```
IEX (New-Object Net.WebClient).DownloadString("https://raw.githubusercontent.com/Drew-Alleman/DataSurgeon/main/install/install.ps1")
```
Relaunch your terminal and you will be able to use ```ds``` from the command line.

### Mac
```
curl --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/Drew-Alleman/DataSurgeon/main/install/install.sh | sh
```

# Command Line Arguments
```
$ ds -h 
DataSurgeon: https://github.com/Drew-Alleman/DataSurgeon 1.0
Drew Alleman
DataSurgeon (ds) extracts sensitive information from standard input for incident response,
penetration testing, and CTF challenges, including emails, credit cards, URLs, IPs, MAC addresses,
and SRV DNS records.

USAGE:
    ds [OPTIONS]

OPTIONS:
    -6, --ipv6-addr          Extracts IPv6 addresses from the desired file
    -c, --credit-card        Extract credit card numbers
    -C, --clean              Attempt to remove some of the clean information that might have been
                             sent back
    -d, --dns                Extract Domain Name System records
    -D, --domain-users       Extract possible Windows domain user accounts
    -e, --email              Used to extract email addresses from the specifed file or output stream
    -f, --file <file>        File to extract information from
    -F, --files              Extract filenames
    -h, --help               Print help information
    -H, --hash               Used to extract supported hashes (NTLM, LM, bcrypt, Oracle, MD5, SHA-1,
                             SHA-224, SHA-256, SHA-384, SHA-512, SHA3-224, SHA3-256, SHA3-384,
                             SHA3-512, MD4) from the specified file or output stream
    -i, --ip-addr            Extracts IP addresses from the desired file
    -m, --mac-addr           Extract's MAC addresses
    -o, --output <output>    Output's the results of the procedure to a local file (recommended for
                             large files)
    -t, --time               Time how long the operation took
    -T, --thorough           Continues searching for all selected matches in each row, even if
                             multiple types of matches are found. By default, the program stops at
                             the first match found in each row. (Slower) (Good for CSV's and JSON
                             files)
    -u, --url                Extract url's
    -V, --version            Print version information
    -X, --hide               Hides the identifier string infront of the desired content (e.g: 'hash:
                             ', 'url: ', 'email: ' will not be displayed.                          
```
# Examples
## Extracting Files From a Remote Webiste
Here I use ```wget``` to make a request to stackoverflow then I forward the body text to ```ds``` . The ```-F``` option will list all files found. ```--clean``` is used to remove any extra text that might have been returned (such as extra html). Then the result of is sent to ```uniq``` which removes any non unique files found.
```
wget -qO - https://www.stackoverflow.com | ds -F --clean | uniq                                                                                      
files: apple-touch-icon.png
files: opensearch.xml
files: 2.png
files: min.js
files: en.js
files: illo-integrations-left.png
files: illo-integrations-right.png
files: apple-touch-icon.png
files: gtag.js
files: ga.i
```

## Extracting Mac Addresses From an Output File
Here I am pulling all mac addresses found in [autodeauth's](https://github.com/Drew-Alleman/autodeauth) log file using the ```-m``` query. The ```--hide``` option will hide the identifer string infront of the results. In this case 'mac_address: ' is hidden from the output.
```
$ ./ds -m --hide -f /var/log/autodeauth/log     
2023-02-26 00:28:19 - Sending 500 deauth frames to network: BC:2E:48:E5:DE:FF -- PrivateNetwork
2023-02-26 00:35:22 - Sending 500 deauth frames to network: 90:58:51:1C:C9:E1 -- TestNet
```

## Reading all files in a directory
The line below will read all files in the current directory then output any important data to ~/output.log
```
find . -type f -exec cat {} \; | ds -C -t -o ~/output.log
```

# Speed Tests
When no specific query is provided, ```ds``` will search through all possible types of data, which is <b>SIGNIFICANTLY</b> slower than using individual queries. The slowest query is ```--files```. Its also slightly faster to use ```cat``` to pipe the data to ```ds```. 

Below is the elapsed time when processing a 5GB test file generated by [ds-test](https://github.com/Drew-Alleman/ds-test). Each test was ran 3 times and the average time was recorded.

### Computer Specs
```
Processor	Intel(R) Core(TM) i5-10400F CPU @ 2.90GHz, 2904 Mhz, 6 Core(s), 12 Logical Processor(s)
Ram         12.0 GB (11.9 GB usable)
```

## Searching all data types

Command         | Speed         
----------------|----------------
`cat test.txt \| ds -t` | 00h:01m:35s |
`ds -t -f test.txt` | 00h:01m:36s
`cat test.txt \| ds -t -o output.txt` | 00h:01m:38s

## Using specific queries

Command         | Speed          | Query Count
----------------|----------------|----------------
`cat test.txt \| ds -t -6` | 00h:00m:13s | 1
`cat test.txt \| ds -t -i -m` | 00h:00m:23 | 2
`cat test.txt \| ds -t -F -6 -c` | 00h:00m:33s | 3

# Project Goals
* CSV output
