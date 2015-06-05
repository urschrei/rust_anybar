// UDP stuff adapted from http://illegalargumentexception.blogspot.co.uk/2015/05/rust-send-and-receive-on-localhost-with.html
use std::thread;
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

fn read_message(socket: net::UdpSocket) -> Vec<u8> {
    let mut buf: [u8; 1] = [0; 1];
    let result = socket.recv_from(&mut buf);
    drop(socket);
    let mut data;
    match result {
        Ok((amt, _)) => {
        data = Vec::from(&buf[0..amt]);
        },
    Err(err) => panic!("Read error: {}", err)
    }
    data
}

pub fn send_message(send_addr: net::SocketAddr, target: net::SocketAddr, data: Vec<u8>) {
    let socket = socket(send_addr);
    let _ = socket.send_to(&data, target);
    drop(socket);
    // match result {
    //     Ok(amt) => println!("Sent {} bytes", amt),
    //     Err(err) => panic!("Write error: {}", err)
    // }
}

pub fn listen(listen_on: net::SocketAddr) -> thread::JoinHandle<Vec<u8>> {
    let socket = socket(listen_on);
    let handle = thread::spawn(move || {
        read_message(socket)
    });
    handle
}

#[cfg(test)]
mod test {
    use std::net;
    use std::thread;
    use super::*;

    #[test]
    fn test_udp() {
        println!("UDP");
        let ip = net::Ipv4Addr::new(127, 0, 0, 1);
        let listen_addr = net::SocketAddrV4::new(ip, 8888);
        let send_addr = net::SocketAddrV4::new(ip, 8889);
        let future = listen(net::SocketAddr::V4(listen_addr));
        let message: Vec<u8> = vec![10];
        // give the thread 3s to open the socket
        thread::sleep_ms(3000);
        send_message(
            net::SocketAddr::V4(send_addr),
            net::SocketAddr::V4(listen_addr),
            message);
        println!("Waiting");
        let received = future.join().unwrap();
        println!("Got {} bytes", received.len());
        assert_eq!(1, received.len());
        assert_eq!(10, received[0]);
    }
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

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options] [command]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let mut opts = Options::new();
    // this waits on a prompt, so not what we want
    // http://stackoverflow.com/a/27973038/416626
    // use std::io;
    // use std::io::prelude::*;
    // let stdin = io::stdin();
    // let inp = stdin
    //     .lock()
    //     .lines()
    //     .next()
    //     .unwrap()
    //     .unwrap()
    //     .trim()
    //     .to_string();
    
    // get command-line input
    // Strings do not live for the entire life of your program
    // http://stackoverflow.com/a/23977218/416626
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    opts.optopt("p", "", "Set destination UDP port", "PORT");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => {m}
        Err(f) => {panic!(f.to_string())}
    };
    // TODO: handle panicking non-integer conversions via a match here
    let port = matches.opt_str("p").unwrap_or("1738".to_string());
    // gather non-option arguments
    let arg = if !matches.free.is_empty() {
        matches.free[0].clone()
    } else {
        print_usage(&program, opts);
        return;
    };
    let intermediate: &str = &*arg;
    
    // match command-line input or bail out horribly
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
        _ => panic!("Unknown option!")
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
    // parse() gives us an int Result which we need to unwrap
    let listen_addr = net::SocketAddrV4::new(ip, port.parse().unwrap());
    let send_addr = net::SocketAddrV4::new(ip, 0);
    // and send our message
    send_message(
        net::SocketAddr::V4(send_addr),
        net::SocketAddr::V4(listen_addr),
        message);
}
