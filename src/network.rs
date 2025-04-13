use std::net::{SocketAddr, ToSocketAddrs, UdpSocket};

pub struct UDPNetwork {
    socket: UdpSocket,
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
        if let Ok((amt, src)) = self.socket.recv_from(&mut buf) {
            Some((String::from_utf8_lossy(&buf[..amt]).to_string(), src))
        } else {
            None
        }
    }

    pub fn send_to<A: ToSocketAddrs>(&self, data: &str, addr: A) {
        self.socket
            .send_to(data.as_bytes(), addr)
            .expect("Unable to send data");
    }

    pub fn send(&self, data: &str) {
        self.socket
            .send(data.as_bytes())
            .expect("Unable to send data");
    }

    pub fn connect<A: ToSocketAddrs>(&self, addr: A) {
        self.socket
            .connect(addr)
            .expect("Couldn't connect to UDP socket");
    }
}
