# Description
This is an app for controlling the [Anybar](https://github.com/tonsky/AnyBar) application.

# Building

- Run `cargo build --release` from the project root  
- The binary will be available as `target/release/anybar_rs`  
- Copy it into your `$PATH`.

# Running it
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
## Specifying an alternate port

# License
MIT

# And also
[![witnessme](witnessme.gif)]( "SHINY AND CHROME")

