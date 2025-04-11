use std::net::SocketAddrV4;
use std::thread::sleep;
use std::time::Duration;
use crate::network::UDPNetwork;

pub struct Client {
    pub network: UDPNetwork,
}
impl Client {
    pub fn new() -> Client {
        let network = UDPNetwork::new();
        Client { network }
    }

    pub fn connect(&self, addr: SocketAddrV4) {
        self.network.connect(addr);
        loop {
            if let Some((data, src)) = self.network.recv_from() {
                println!("Received: {} from {}", data, src);
                if data == "okay" {
                    self.network.send("0*AUTH*")
                } else if data == "pong" {
                    break;
                }
            } else {
                println!("[UDP] Sending ping...");
                self.network.send("ping")
            }

            sleep(Duration::from_secs(3));
        }

        println!("[UDP] Connected to server");
    }

    pub fn send_sensation(&self) {
        self.network.send("0*SENSATION*100,10,30,0,0,0,test|2%100")
    }
}
