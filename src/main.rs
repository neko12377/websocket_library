use std::io::{prelude::*, BufReader};
use std::net::{
    SocketAddr,
    TcpListener,
    TcpStream,
};

fn main() -> std::io::Result<()>{

    let addrs = [
        SocketAddr::from(([127, 0, 0, 1], 17888)),
    ];

    let listener = TcpListener::bind(&addrs[..]).expect("Should be listener");

    // loop {
    match listener.accept() {
        Ok((socket, addr)) => {
            println!("new client: {addr:?}");
            println!("stream is {socket:?}");
            handle_connection(socket);
        },
        Err(e) => println!("couldn't get client: {e:?}"),
    };
    // }
    Ok(())
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
    println!("{key:?}");
    socket.write(b"response").expect("Should repound to incoming request");
}
