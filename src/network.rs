use std::net::{SocketAddr, SocketAddrV4, UdpSocket};

pub struct UDPNetwork {
    socket: UdpSocket
}
impl UDPNetwork {
    pub fn new() -> UDPNetwork {
        let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
        socket.set_broadcast(true).unwrap();
        socket.set_nonblocking(true).unwrap();

        UDPNetwork { socket }
    }

    pub fn recv_from(&self) -> Option<(String, SocketAddr)> {
        let mut buf = [0; 1024];
        println!("[UDP] Waiting for data...");
        if let Ok((amt, src)) = self.socket.recv_from(&mut buf) {
            println!("[UDP] Received {} bytes from {}", amt, src);
            Some((String::from_utf8_lossy(&buf[..amt]).to_string(), src))
        } else {
            println!("[UDP] Got nothing");
            None
        }
    }

    pub fn send(&self, data: &str) {
        self.socket.send(data.as_bytes()).expect("Unable to send data");
    }

    pub fn connect(&self, addr: SocketAddrV4) {
        self.socket.connect(addr).expect("Couldn't connect to UDP socket");
    }
}
