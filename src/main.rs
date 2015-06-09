// UDP stuff adapted from http://illegalargumentexception.blogspot.co.uk/2015/05/rust-send-and-receive-on-localhost-with.html
use std::net;
use std::env;
use std::convert::AsRef;
extern crate getopts;
use getopts::Options;

fn socket(listen_on: net::SocketAddr) -> net::UdpSocket {
    let attempt = net::UdpSocket::bind(listen_on);
    let mut socket;
    match attempt {
        Ok(sock) => {
            socket = sock;
        },
    Err(err) => panic!("Could not bind: {}", err)
    }
    socket
}

pub fn send_message(send_addr: net::SocketAddr, target: net::SocketAddr, data: Vec<u8>) {
    let socket = socket(send_addr);
    let _ = socket.send_to(&data, target);
    drop(socket);
}

fn convert(inp: &str) -> String {
    // to_lowercase is still unstable.
    // This madness from IRC is the workaround
    // works though
    let outp = inp
        .chars()
        .flat_map(char::to_lowercase)
        .collect::<String>();
    outp
}

fn print_usage(program: &str, opts: Options, ecode: i32) {
    let brief = format!("Usage: {} [options] [command]", program);
    print!("{}", opts.usage(&brief));
    std::process::exit(ecode);
}

fn main() {
    let mut opts = Options::new();
    // get command-line input
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    opts.optopt("p", "port", "Set destination UDP port. Must be an integer.", "PORT");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => {m}
        Err(f) => {panic!(f.to_string())}
    };
    // Get port from the option, or specify the default
    let port = matches.opt_str("p").unwrap_or("1738".to_string());
    // cast to int and ensure it worked, or set an error flag
    let numeric_port = port.parse::<u16>();
    let proceed = match numeric_port {
        Ok(_) => true,
        Err(_) => false
    };
    let arg = if !matches.free.is_empty() && proceed {
        matches.free[0].clone()
    } else {
        print_usage(&program, opts, 1);
        return;
    };
    // Strings do not live for the entire life of your program
    // http://stackoverflow.com/a/23977218/416626
    let intermediate: &str = &*arg;
    // match command-line input or print usage
    let to_send = match convert(intermediate).as_ref() {
        "white" => "white".to_string(),
        "red" => "red".to_string(),
        "orange" => "orange".to_string(),
        "yellow" => "yellow".to_string(),
        "green" => "green".to_string(),
        "cyan" => "cyan".to_string(),
        "blue" => "blue".to_string(),
        "purple" => "purple".to_string(),
        "black" => "black".to_string(),
        "?" => "question".to_string(),
        "!" => "exclamation".to_string(),
        "quit" => "quit".to_string(),
        _ => {
            print_usage(&program, opts, 1);
            return;
        }
    };
    // blam our control message into a vector 
    let mut message: Vec<u8> = Vec::new();
    message.extend(to_send
        .as_bytes()
        .iter()
        .cloned()
    );
    // bind to the correct UDP port
    let ip = net::Ipv4Addr::new(127, 0, 0, 1);
    // parse() gives us Result which we need to unwrap
    // it's safe to unwrap here, cos we already checked success 
    let listen_addr = net::SocketAddrV4::new(ip, numeric_port.unwrap());
    let send_addr = net::SocketAddrV4::new(ip, 0);
    // and send our message
    send_message(
        net::SocketAddr::V4(send_addr),
        net::SocketAddr::V4(listen_addr),
        message);
}
