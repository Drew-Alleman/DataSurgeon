# DataSurgeon
![preview](media/main.gif)
DataSurgeon (ds) is a versatile tool designed for incident response, penetration testing, and CTF challenges. It allows for the extraction of various types of sensitive information including emails, phone numbers, hashes, credit cards, URLs, IP addresses, MAC addresses, SRV DNS records and a lot more!

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
* Phone numbers
* Credit Cards
* Google API Private Key ID's
* Social Security Numbers
* AWS Keys
* Bitcoin wallets
* URL's
* IPv4 Addresses and IPv6 addresses
* MAC Addresses
* SRV DNS Records
* Extract Hashes
    - MD4 & MD5
    - SHA-1, SHA-224, SHA-256, SHA-384, SHA-512
    - SHA-3 224, SHA-3 256, SHA-3 384, SHA-3 512
    - MySQL 323, MySQL 41
    - NTLM
    - bcrypt

### Want more? 
Please read the contributing guidelines [here](https://github.com/Drew-Alleman/DataSurgeon/blob/main/CONTRIBUTING.md#adding-a-new-regex--extraction-feature)

# Quick Install
To install DataSurgeon, you need to install [Rust](https://www.rust-lang.org/tools/install) and [GitHub](https://desktop.github.com/).
### Linux
```
wget -O - https://raw.githubusercontent.com/Drew-Alleman/DataSurgeon/main/install/install.sh | bash
```

### Windows 
Run the following command in an elevated PowerShell window:
```
IEX (New-Object Net.WebClient).DownloadString("https://raw.githubusercontent.com/Drew-Alleman/DataSurgeon/main/install/install.ps1")
```
After installing, restart your terminal, and you can use ```ds``` from the command line.

### Mac
```
curl --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/Drew-Alleman/DataSurgeon/main/install/install.sh | sh
```

# Command Line Arguments
```
$ ds -h 
Note: All extraction features (e.g: -i) work on a specified file (-f) or an output stream.

Usage: ds [OPTIONS]

Options:
  -f, --file <file>      File to extract information from
  -C, --clean            Only displays the matched result, rather than the entire line
  -T, --thorough         Doesn't stop at first match (useful for -C if multiple unique matches are on the same line
  -D, --display           Displays the filename assoicated with the content found (https://github.com/Drew-Alleman/DataSurgeon#reading-all-files-in-a-directory)
  -X, --hide             Hides the identifier string infront of the desired content (e.g: 'hash: ', 'url: ', 'email: ' will not be displayed.
  -o, --output <output>  Output's the results of the procedure to a local file (recommended for large files)
  -t, --time             Time how long the operation took
  -e, --email            Extract email addresses
  -p, --phone            Extracts phone numbers
  -H, --hash             Extract hashes (NTLM, LM, bcrypt, Oracle, MD5, SHA-1, SHA-224, SHA-256, SHA-384, SHA-512, SHA3-224, SHA3-256, SHA3-384, SHA3-512, MD4)
  -i, --ip-addr          Extract IP addresses
  -6, --ipv6-addr        Extract IPv6 addresses
  -m, --mac-addr         Extract MAC addresses
  -c, --credit-card      Extract credit card numbers
  -u, --url              Extract urls
  -F, --files            Extract filenames
  -b, --bitcoin          Extract bitcoin wallets
  -a, --aws              Extract AWS keys
  -g, --google           Extract Google service account private key ids (used for google automations services)
  -d, --dns              Extract Domain Name System records
  -s, --social           Extract social security numbers
  -h, --help             Print help
  -V, --version          Print version                         
```
# Examples
## Extracting Files From a Remote Webiste
Here I use ```wget``` to make a request to stackoverflow then I forward the body text to ```ds``` . The ```-F``` option will list all files found. ```--clean``` is used to remove any extra text that might have been returned (such as extra html). Then the result of is sent to ```uniq``` which removes any non unique files found.
```
 wget -qO - https://www.stackoverflow.com | ds -F --clean | uniq
```
![preview](media/wget_preview.gif)

## Extracting Mac Addresses From an Output File
I am extracting all MAC addresses found in [autodeauth's](https://github.com/Drew-Alleman/autodeauth) log file using the ```-m``` query. To make the output cleaner, I'm using the ```--hide``` option to remove the identifier string 'mac_address: ' from the results. Additionally, I'm using the ```-T``` option to allow the tool to check the same line multiple times for matches. By default, the tool moves on to the next line after a match is found, but with ```-T``` it will keep searching for additional unique matches on the same line.
```
$ ./ds -m -T --hide -f /var/log/autodeauth/log     
2023-02-26 00:28:19 - Sending 500 deauth frames to network: BC:2E:48:E5:DE:FF -- PrivateNetwork
2023-02-26 00:35:22 - Sending 500 deauth frames to network: 90:58:51:1C:C9:E1 -- TestNet
```

## Reading all files in a directory
The command below recursively reads all files in the current directory. Use the ```-D``` option to display the filename (requires ```-f``` to show the filename) and the ```-e``` option to search for email addresses.
```
$ find . -type f -exec ds -f {} -CDe \;
```
![preview](media/directory_search.gif)


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
`cat test.txt \| ds -t` | 00h:02m:04s |
`ds -t -f test.txt` | 00h:02m:05s
`cat test.txt \| ds -t -o output.txt` | 00h:02m:06s

## Using specific queries

Command         | Speed          | Query Count
----------------|----------------|----------------
`cat test.txt \| ds -t -6` | 00h:00m:12s | 1
`cat test.txt \| ds -t -i -m` | 00h:00m:22 | 2
`cat test.txt \| ds -tF6c` | 00h:00m:32s | 3

# Project Goals
* JSON and CSV output
