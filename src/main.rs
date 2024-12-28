// UDP stuff adapted from http://illegalargumentexception.blogspot.co.uk/2015/05/rust-send-and-receive-on-localhost-with.html
use clap::{crate_version, value_parser, Arg, Command};
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4, UdpSocket};

fn socket(listen_on: SocketAddr) -> UdpSocket {
    match UdpSocket::bind(listen_on) {
        Ok(sock) => sock,
        Err(err) => panic!("Could not bind: {}", err),
    }
}

fn send_message(send_addr: SocketAddr, target: SocketAddr, data: &[u8]) {
    socket(send_addr)
        .send_to(data, target)
        .expect("Couldn't send message.");
}

fn main() {
    let command_vals = [
        "white",
        "red",
        "orange",
        "yellow",
        "green",
        "cyan",
        "blue",
        "purple",
        "black",
        "question",
        "exclamation",
        "filled",
        "hollow",
        "quit",
    ];
    let command_params = Command::new("rust_anybar")
        .version(crate_version!())
        .author("Stephan Hügel <urschrei@gmail.com>")
        .about("A Rust command-line client for Anybar 0.2.3")
        .arg(
            Arg::new("PORT")
                .short('p')
                .long("port")
                .help("Set destination UDP port. Input must be 0–65535")
                .value_parser(value_parser!(u16).range(0..=65535))
                .default_value("1738"),
        )
        .arg(
            Arg::new("COMMAND")
                .help("The command you wish to send to Anybar")
                .index(1)
                .required(true)
                // Use value_parser to supply valid possible values
                .value_parser(command_vals),
        )
        .get_matches();
    // bind to the correct UDP port
    let numeric_port: u16 = *command_params
        .get_one::<u16>("PORT")
        .expect("Failed to get port");
    let command = command_params
        .get_one::<String>("COMMAND")
        .expect("COMMAND argument missing");
    // and send our message
    send_message(
        SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 0)),
        SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), numeric_port)),
        command.as_bytes(),
    );
}
