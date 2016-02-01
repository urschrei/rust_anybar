[![](https://img.shields.io/crates/v/anybar_rs.svg)](https://crates.io/crates/anybar_rs) [![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](license.txt)  

# Description
`anybar_rs` is a command-line app for controlling the [Anybar](https://github.com/tonsky/AnyBar) application.

# Installation
- [Install Rust](https://www.rust-lang.org/downloads.html) or [Multirust](https://github.com/brson/multirust)
- Ensure that the Cargo binary location is in your `$PATH`:
    - e.g. `/Users/urschrei/.cargo/bin`
    - You can specify an alternate location by passing `--root DIR` to `cargo install`
- Run `cargo install anybar_rs`
- The binary should now be available to use. To check, run `anybar_rs --help`.

# Building from Source
- [Install Rust](https://www.rust-lang.org/downloads.html) or [Multirust](https://github.com/brson/multirust), and ensure that they're in your `$PATH`
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
- ?
- !
- quit


# Exit codes
Being a stateless protocol, UDP provides no mechanism for checking whether its datagrams arrive.

- If the UDP datagram is succesfully *sent*, `anybar_rs` will exit with code `0`.
- Unknown flags, options, or an unknown `command` will print the usage and exit with code `1`.

# License
MIT

# And also
[![witnessme](witnessme.gif)]( "SHINY AND CHROME")

