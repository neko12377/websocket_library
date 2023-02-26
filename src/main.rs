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
            Ok((stream, _addr)) => {
                handle_connection(stream);
            },
            Err(e) => println!("couldn't get client: {e:?}"),
        };
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::with_capacity(8, &mut stream);
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
        handle_stream_response(stream, key);
    } else {
        println!("Aborting non-WebSocket connection");
        stream.write(b"response").expect("Should repound to incoming request");
        return
    }

}

fn handle_stream_response(mut stream: TcpStream, key: &str) {
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

    stream.write_all(
        format!("HTTP/1.1 101 Switching Protocols\r\nUpgrade: websocket\r\nConnection: Upgrade\r\nSec-WebSocket-Accept: {}\r\n\r\n", res).as_bytes()
    ).expect("Should write handshake response");
    let _ = read_websocket_message(stream);
}

fn read_websocket_message(mut stream: TcpStream) -> std::io::Result<()> {
    let mut buffer = [0u8; 2];
    stream.read(&mut buffer)?;
    let (first, second) = (buffer[0], buffer[1]);
    
    let _is_final = first & 0x80 != 0;

    let _rsv1 = first & 0x40 != 0;
    let _rsv2 = first & 0x20 != 0;
    let _rsv3 = first & 0x10 != 0;

    let opcode = match_opcode(first & 0x0F);
    println!("OpCode is {opcode:?}");

    let _masked = second & 0x80 != 0;
    
    let payload_size = u64::from(second & 0x7F);
    println!("Payload size is {payload_size:?}");
   
    let mut mask = [0u8; 4];
    stream.read(&mut mask)?;
   
    let mut data = vec![0u8; payload_size as usize];
    stream.read(&mut data)?;

    println!("Masked data is {data:?}");
   
    let unmasked_data: Vec<u8> = data
        .iter()
        .enumerate()
        .map(|(i, &byte)| byte ^ mask[i % 4])
        .collect();

    println!("Unmasked data is {unmasked_data:?}");

    println!("Message: {:?}", String::from_utf8_lossy(&unmasked_data));
    Ok(())
}

fn match_opcode(byte: u8) -> &'static str {
    match byte {
        0 => "continuation",
        1 => "test",
        2 => "binary",
        3..=7 => "reserved for further non-control frames",
        8 => "connection closed",
        9 => "ping",
        10 => "pong",
        11..=15 => "reserved for further control frames",
        _ => panic!("out of range"),
    }
}

