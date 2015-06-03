use std::thread;
use std::net;
use std::env;
use std::convert::AsRef;

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
    let outp = inp
        .chars()
        .flat_map(char::to_lowercase)
        .collect::<String>();
    outp
}

fn main() {
    // get command-line input

    // this waits on a prompt, so
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

    // Strings do not live for the entire life of your program
    // http://stackoverflow.com/a/23977218/416626
    let args: Vec<String> = env::args().collect();
    let arg = args[1].clone();
    let intermediate: &str = &*arg;

    // match input or bail out horribly
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

    let mut message: Vec<u8> = Vec::new();
    message.extend(to_send
        .as_bytes()
        .iter()
        .cloned()
    );
    let ip = net::Ipv4Addr::new(127, 0, 0, 1);
    let listen_addr = net::SocketAddrV4::new(ip, 1738);
    let send_addr = net::SocketAddrV4::new(ip, 0);
    send_message(
        net::SocketAddr::V4(send_addr),
        net::SocketAddr::V4(listen_addr),
        message);
}
