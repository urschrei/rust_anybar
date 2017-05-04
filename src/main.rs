// UDP stuff adapted from http://illegalargumentexception.blogspot.co.uk/2015/05/rust-send-and-receive-on-localhost-with.html
use std::net::{SocketAddr, SocketAddrV4, Ipv4Addr, UdpSocket};
#[macro_use]
extern crate clap;
use clap::{Arg, App};

fn socket(listen_on: SocketAddr) -> UdpSocket {
    match UdpSocket::bind(listen_on) {
        Ok(sock) => sock,
        Err(err) => panic!("Could not bind: {}", err),
    }
}

fn send_message(send_addr: SocketAddr, target: SocketAddr, data: &[u8]) {
    socket(send_addr).send_to(data, target).expect("Couldn't send message.");
}

fn main() {
    let command_vals = ["white",
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
                        "quit"];
    let command_params = App::new("rust_anybar")
        .version(&crate_version!()[..])
        .author("Stephan Hügel <urschrei@gmail.com>")
        .about("A Rust command-line client for Anybar")
        .args_from_usage("-p --port=[PORT] 'Set destination UDP port. Input must be 0 – 65535, \
                          and defaults to 17388")
        .arg(Arg::with_name("COMMAND")
            .help("The command you wish to send to Anybar")
            .index(1)
            .possible_values(&command_vals)
            .required(true))
        .get_matches();
    // bind to the correct UDP port
    let numeric_port = value_t!(command_params.value_of("PORT"), u16).unwrap_or(1738);
    // it's safe to unwrap here, cos we already checked success
    let to_send = command_params.value_of("COMMAND").unwrap();
    let ip = Ipv4Addr::new(127, 0, 0, 1);
    let listen_addr = SocketAddrV4::new(ip, numeric_port);
    let send_addr = SocketAddrV4::new(ip, 0);
    // and send our message
    send_message(SocketAddr::V4(send_addr),
                 SocketAddr::V4(listen_addr),
                 to_send.as_bytes());
}
