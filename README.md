# png-message
A CLI tool to encode and decode messages inside PNG files. 

# Installation
You will need the Rust toolchain to install this application. 

```
$ git clone https://github.com/hedonhermdev/png-message
$ cargo install --path=png-message/
```

# Usage
```
$ pngme 
PngMe 0.1.0
Encode/Decode secret messages in PNG files

USAGE:
    pngme <SUBCOMMAND>
FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    decode    Decode a secret message from the given file
    encode    Encode a secret message in the given file
    help      Prints this message or the help of the given subcommand(s)
    remove    Remove a chunk from the given file
```
