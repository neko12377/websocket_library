use std::net::{
    SocketAddr,
    TcpListener,
};
use websocket_server::connection_handler::handle_connection;

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
