[![](https://img.shields.io/crates/v/anybar_rs.svg)](https://crates.io/crates/anybar_rs) [![MIT Licensed](https://img.shields.io/github/license/mashape/apistatus.svg)](license.txt)  

# Description
`anybar_rs` is a command-line app for controlling the MacOS[Anybar](https://github.com/tonsky/AnyBar) application.

# Installation
- Download the latest 64-bit or 32-bit version [here](https://github.com/urschrei/rust_anybar/releases/latest)
    - unzip, then copy the binary into your `$PATH` (e.g. `/usr/local/bin`)
- Alternatively:
    - install Rust and Cargo. Using [rustup.rs](https://www.rustup.rs) is the easiest way to do this
    - run `cargo install anybar_rs`
- The binary should now be available to use. To check, run `anybar_rs --help`.

# Building from Source
- Clone this project
- Run `cargo build --release` from the project root
- The binary will be available as `target/release/anybar_rs`
- Copy it into your `$PATH`.

# Usage
By default, `anybar_rs` assumes that Anybar is bound to localhost on UDP port 1738. Call it like so:  
`anybar_rs [FLAGS] [OPTIONS] <COMMAND>`  
**FLAGS**:  
    `-h`, `--help`       Prints help information  
    `-V`, `--version`    Prints version information  

**OPTIONS**:  
    `-p`, `--port` <PORT>    Set destination UDP port. Input must be 0 – 6553, and defaults to 1738  

**ARGS**:  
    `COMMAND`    The command you wish to send to Anybar  

Where `COMMAND` is one of the following:

- white
- red
- orange
- yellow
- green
- cyan
- blue
- purple
- black
- question
- exclamation
- quit


# Exit codes
Being a stateless protocol, UDP provides no mechanism for checking whether its datagrams arrive.

- If the UDP datagram is succesfully *sent*, `anybar_rs` will exit with code `0`.
- Unknown flags, options, or an unknown `command` will print the usage and exit with code `1`.

# License
MIT
