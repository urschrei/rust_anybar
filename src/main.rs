// UDP stuff adapted from http://illegalargumentexception.blogspot.co.uk/2015/05/rust-send-and-receive-on-localhost-with.html
use std::net;
#[macro_use]
extern crate clap;
use clap::{Arg, App};

fn socket(listen_on: net::SocketAddr) -> net::UdpSocket {
    let attempt = net::UdpSocket::bind(listen_on);
    let socket;
    match attempt {
        Ok(sock) => {
            socket = sock;
        }
        Err(err) => panic!("Could not bind: {}", err),
    }
    socket
}

pub fn send_message(send_addr: net::SocketAddr, target: net::SocketAddr, data: Vec<u8>) {
    let socket = socket(send_addr);
    let _ = socket.send_to(&data, target);
    drop(socket);
}

fn convert(inp: String) -> String {
    // to_lowercase is still unstable.
    // This madness from IRC is the workaround
    // works though

    // Strings do not live for the entire life of your program
    // http://stackoverflow.com/a/23977218/416626
    let intermediate: &str = &*inp;
    let outp = intermediate.chars()
                           .flat_map(char::to_lowercase)
                           .collect::<String>();
    outp
}

fn main() {
    let command_vals = ["white", "red", "orange", "yellow", "green", "cyan", "blue", "purple",
                        "black", "?", "!", "quit"];
    let matches = App::new("rust_anybar")
                      .version(&crate_version!()[..])
                      .author("Stephan Hügel <urschrei@gmail.com>")
                      .about("A Rust command-line client for Anybar")
                      .args_from_usage("-p --port=[PORT] 'Set destination UDP port. Input must \
                                        be 0 – 65535")
                      .arg(Arg::with_name("COMMAND")
                               .help("The command you wish to send to Anybar")
                               .index(1)
                               .possible_values(&command_vals)
                               .required(true))
                      .get_matches();
    let port = matches.value_of("PORT").unwrap_or("1738");
    let to_send = convert(matches.value_of("COMMAND").unwrap().to_string());
    // // cast to int and ensure it worked
    let numeric_port = port.parse::<u16>();
    // blam our control message into a vector
    let mut message: Vec<u8> = Vec::new();
    message.extend(to_send.as_bytes()
                          .iter()
                          .cloned());
    // bind to the correct UDP port
    let ip = net::Ipv4Addr::new(127, 0, 0, 1);
    // parse() gives us Result which we need to unwrap
    // it's safe to unwrap here, cos we already checked success
    let listen_addr = net::SocketAddrV4::new(ip, numeric_port.unwrap());
    let send_addr = net::SocketAddrV4::new(ip, 0);
    // and send our message
    send_message(net::SocketAddr::V4(send_addr),
                 net::SocketAddr::V4(listen_addr),
                 message);
}
