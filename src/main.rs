// UDP stuff adapted from http://illegalargumentexception.blogspot.co.uk/2015/05/rust-send-and-receive-on-localhost-with.html
use clap::{arg, crate_version, App, Arg};
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
    let command_params = App::new("rust_anybar")
        .version(crate_version!())
        .author("Stephan Hügel <urschrei@gmail.com>")
        .about("A Rust command-line client for Anybar 0.2.3")
        .arg(
            arg!(-p --port <PORT> "Set destination UDP port. Input must be 0 – 65535")
                .default_value("1738")
                .required(false),
        )
        .arg(
            Arg::new("COMMAND")
                .help("The command you wish to send to Anybar")
                .index(1)
                .possible_values(command_vals)
                .required(true),
        )
        .get_matches();
    // bind to the correct UDP port
    let numeric_port: u16 = command_params.value_of_t("PORT").unwrap();
    // and send our message
    send_message(
        SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 0)),
        SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), numeric_port)),
        command_params.value_of("COMMAND").unwrap().as_bytes(),
    );
}
