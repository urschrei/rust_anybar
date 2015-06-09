# Description
`anybar_rs` is a command-line app for controlling the [Anybar](https://github.com/tonsky/AnyBar) application.

# Building

- Ensure you've installed Rust 1.x and Cargo, and they're in your `$PATH`
- Run `cargo build --release` from the project root
- The binary will be available as `target/release/anybar_rs`
- Copy it into your `$PATH`.

# Usage
By default, `anybar_rs` assumes that Anybar is bound to localhost on UDP port 1738. Call it like so:  
`anybar_rs [command]`  
You can send to a different port by calling the program with the `-p` or `--port` option:  
`anybar_rs -p --port [number] [command]`

Where `command` is one of the following:

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

(For the sake of practicality, uppercase and mixed-case input is accepted)

# Exit codes
Being a stateless protocol, UDP provides no mechanism for checking whether its datagrams arrive.

- If the UDP datagram is succesfully *sent*, `anybar_rs` will exit with code `0`.
- An unknown command, or incorrect usage of `-p` will print usage and exit with code `1`.

# License
MIT

# And also
[![witnessme](witnessme.gif)]( "SHINY AND CHROME")

