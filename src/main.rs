// UDP stuff adapted from http://illegalargumentexception.blogspot.co.uk/2015/05/rust-send-and-receive-on-localhost-with.html
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4, UdpSocket};
#[macro_use]
extern crate clap;
use clap::{App, Arg};

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
        "quit",
    ];
    let command_params = App::new("rust_anybar")
        .version(&crate_version!()[..])
        .author("Stephan Hügel <urschrei@gmail.com>")
        .about("A Rust command-line client for Anybar")
        .args_from_usage(
            "-p --port=[PORT] 'Set destination UDP port. Input must be 0 – 65535, \
             and defaults to 17388",
        )
        .arg(
            Arg::with_name("COMMAND")
                .help("The command you wish to send to Anybar")
                .index(1)
                .possible_values(&command_vals)
                .required(true),
        )
        .get_matches();
    // bind to the correct UDP port
    let numeric_port = value_t!(command_params.value_of("PORT"), u16).unwrap_or(1738);
    // and send our message
    send_message(
        SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 0)),
        SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), numeric_port)),
        command_params.value_of("COMMAND").unwrap().as_bytes(),
    );
}
