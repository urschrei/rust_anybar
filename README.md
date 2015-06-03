# Description
This is an app for controlling the [Anybar](https://github.com/tonsky/AnyBar) application.

# Building

- Run `cargo build --release` from the project root  
- The binary will be available as `target/release/anybar_rs`  
- Copy it into your `$PATH`.

# Running it
`anybar_rs` assumes that Anybar is bound to localhost on UDP port 1738. You can alter this by changing the `listen_addr` value in the `main()` function within [main.rs](main.rs).  
Call it like so: `anybar_rs [command]`, where `command` is one of the following (for the sake of practicality, uppercase and mixed-case input is accepted):

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

# License
MIT

