use std::io::{prelude::*, BufReader};
use std::net::{
    SocketAddr,
    TcpListener,
    TcpStream,
};
use sha1::{ Digest, Sha1 };
use base64::{Engine as _, engine::general_purpose};

fn main() -> std::io::Result<()>{

    let addrs = [
        SocketAddr::from(([127, 0, 0, 1], 17888)),
    ];

    let listener = TcpListener::bind(&addrs[..]).expect("Should be listener");

    loop {
        match listener.accept() {
            Ok((socket, addr)) => {
                println!("new client: {addr:?}");
                println!("stream is {socket:?}");
                handle_connection(socket);
            },
            Err(e) => println!("couldn't get client: {e:?}"),
        };
    }
}

fn handle_connection(mut socket: TcpStream) {
    let buf_reader = BufReader::with_capacity(8, &mut socket);
    let request = buf_reader
        .lines()
        .map(|result| result.expect("Should be request"))
        .take_while(|line| !line.is_empty())
        .inspect(|x| println!("{x:?}"))
        .collect::<Vec<_>>();

    let key = request
        .iter()
        .filter(|h| h.contains("Sec-WebSocket-Key"))
        .collect::<Vec<_>>();
    if !key.is_empty() {
        println!("WebSocket handshake detected with key: {key:?}");
        let key = key[0].split(" ").collect::<Vec<&str>>()[1];
        println!("key vlaue = {key:?}");
        handle_socket_response(&mut socket, key);
    } else {
        println!("Aborting non-WebSocket connection");
        socket.write(b"response").expect("Should repound to incoming request");
        return
    }

}

fn handle_socket_response(socket:&mut TcpStream, key: &str) {
    // Concatenate key with 258EAFA5-E914-47DA-95CA-C5AB0DC85B11 (RFC 6455)
    // Take SHA-1 hash of this string to obtain hash value
    const WS_GUID: &[u8] = b"258EAFA5-E914-47DA-95CA-C5AB0DC85B11";
    let key = key.as_bytes();
    let mut hasher = Sha1::default();
    hasher.update(key);
    hasher.update(WS_GUID);
    // Encode hash value with base-64  
    let res = general_purpose::STANDARD.encode(&hasher.finalize());
    println!("Sec-WebSOcket-Accept: {res:?}");

    socket.write_all(
        format!("HTTP/1.1 101 Switching Protocols\r\nUpgrade: websocket\r\nConnection: Upgrade\r\nSec-WebSocket-Accept: {}\r\n\r\n", res).as_bytes()
    ).expect("Should write handshake response");
}
