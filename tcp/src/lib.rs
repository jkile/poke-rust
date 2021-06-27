use std::net::{Ipv4Addr, SocketAddrV4, TcpListener, TcpStream};

pub fn init_listener() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    match listener.accept() {
        Ok((_socket, addr)) => println!("New client: {:?}", addr),
        Err(e) => println!("Couldn't get client: {:?}", e),
    }
}

pub fn init_connection(addr: SocketAddrV4) {
    if let Ok(_) = TcpStream::connect(addr) {
        println!("Connected to the server!");
    } else {
        println!("Could not connect to server");
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        super::init_connection();
    }
}
