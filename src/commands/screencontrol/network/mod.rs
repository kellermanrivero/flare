use std::net::{SocketAddr, UdpSocket};

pub struct UdpClient {
    server_address: SocketAddr,
    socket: UdpSocket,
}

impl UdpClient {
    pub fn create(server_port: u16) -> Self {
        let server_address = SocketAddr::from(([127, 0, 0, 1], server_port));
        let socket = UdpSocket::bind("127.0.0.1:0").expect("Cannot bind to socket");
        Self {
            server_address,
            socket,
        }
    }
    pub fn connect(&self) {}

    pub fn send(&self, data: Vec<u8>) {
        let _ = match self.socket.send_to(&data, &self.server_address) {
            Ok(_) => {
                //println!("{} bytes send", bytes_send);
            }
            Err(e) => {
                println!("Cannot send data to the server. Error: {}", e);
            }
        };
    }
}