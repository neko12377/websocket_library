use std::net::{SocketAddr, TcpListener};

fn main() -> std::io::Result<()> {

    let addrs = [
        SocketAddr::from(([127, 0, 0, 1], 17888)),
    ];

    let listener = TcpListener::bind(&addrs[..])?;

    // loop {
    match listener.accept() {
        Ok((_socket, addr)) => println!("new client: {addr:?}"),
        Err(e) => println!("couldn't get client: {e:?}"),
    };
    // }
    Ok(())
}
