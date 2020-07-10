# pngcrypt-rs

![Rust](https://github.com/disDeal/pngcrypt-rs/workflows/Rust/badge.svg)

## Installation

At this point I don't want to publish to crates.io, so the only way to
install the application is to download the source code and build locally.

```Bash
# or through ssh git@github.com:disDeal/pngcrypt-rs.git
git clone https://github.com/disDeal/pngcrypt-rs.git
cd pngcrypt-rs
cargo install --path .
```

## Help

```Bash
cargo run -q -- -h
pngme 0.1.0
Command line program that lets you hide secret messages in PNG files

USAGE:
    pngme <input> <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    decode    Searches for a message hidden in a PNG file and prints
              the message if one is found
    encode    Encodes a message into a PNG file and saves the result
    help      Prints this message or the help of the given subcommands
    print     Prints all of the chunks in a PNG file
    remove    Removes a chunk from a PNG file and saves the result
```

## Usage

```Bash
cargo run -q -- pic.png encode RuST "Lorem ipsum dolor sit amet"

cargo run -q -- pic.png decode RuST
Hidden message in the chunk RuST: 'Lorem ipsum dolor sit amet'

cargo run -q -- pic.png print 
File: pic.png, Size: 4533476

(1) Type: IHDR
  Data size: 13 bytes
  Crc: 3047955392

(2) Type: IDAT
  Data size: 4533381 bytes
  Crc: 95264671

(3) Type: IEND
  Data size: 0 bytes
  Crc: 2923585666

(4) Type: RuST
  Data size: 26 bytes
  Crc: 464893539

cargo run -q -- pic.png remove RuST
```

## Licence

[MIT licenced](LICENCE)
