use std::net::{SocketAddr, UdpSocket};

pub struct UDPNetwork {
    socket: UdpSocket
}
impl UDPNetwork {
    pub fn new() -> UDPNetwork {
        let socket = UdpSocket::bind("127.0.0.1:0").unwrap();
        socket.set_broadcast(true).unwrap();

        UDPNetwork { socket }
    }

    pub fn recv_from(&self) -> Option<(String, SocketAddr)> {
        let mut buf = [0; 1024];
        if let Ok((amt, src)) = self.socket.recv_from(&mut buf) {
            Option::Some((String::from_utf8_lossy(&buf[..amt]).to_string(), src))
        } else {
            Option::None
        }
    }

    pub fn send(&self, data: &str) {
        self.socket.send(data.as_bytes()).unwrap();
    }
}
